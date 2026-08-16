//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 703/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk703<F: Float>(t171: F, t4562: F, t1355: F, t169: F, t700: F, t1383: F, t770: F, t289: F, t4598: F, t274: F, t413: F, t39: F, t745: F) -> (F, F, F, F, F, F) {
    let t5718 = t171 * t4562;
    let t5723 = t169 * t1355 * t700;
    let t5726 = t169 * t770 * t1383;
    let t5730 = F::cast_from(0.31835665774679373271e-1_f64) * t169 * t289 * t4598;
    let t5732 = F::cast_from(0.12798016258123051272e1_f64) * t413 * t274;
    let t5733 = t39 * t745;
    (t5718, t5723, t5726, t5730, t5732, t5733)
}
