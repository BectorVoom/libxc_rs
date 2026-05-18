//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1050/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1050<F: Float>(t1332: F, t274: F, t169: F, t18411: F, t289: F, t4598: F, t766: F, t242: F, t6054: F, t299: F, t4562: F, t5708: F, t700: F) -> (F, F, F, F, F, F) {
    let t18995 = F::new(0.6399008129061525636e1) * t1332 * t274;
    let t18998 = F::new(0.31835665774679373271e-1) * t169 * t289 * t18411;
    let t19001 = F::new(0.2122377718311958218e0) * t169 * t766 * t4598;
    let t19004 = F::new(0.24210827305188264118e1) * t169 * t6054 * t242;
    let t19007 = t169 * t299 * t4562 * t242;
    let t19010 = t169 * t5708 * t700;
    (t18995, t18998, t19001, t19004, t19007, t19010)
}
