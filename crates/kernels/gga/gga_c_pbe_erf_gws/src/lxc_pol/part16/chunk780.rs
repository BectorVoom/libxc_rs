//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 780/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk780<F: Float>(t1354: F, t299: F, t169: F, t242: F, t1343: F, t700: F, t1383: F, t766: F, t1355: F, t770: F, t289: F, t4598: F) -> (F, F, F, F, F, F) {
    let t5708 = t299 * t1354;
    let t5710 = t169 * t5708 * t242;
    let t5713 = t169 * t1343 * t700;
    let t5717 = F::new(0.15917832887339686635e0) * t169 * t766 * t1383;
    let t5723 = t169 * t1355 * t700;
    let t5726 = t169 * t770 * t1383;
    let t5730 = F::new(0.31835665774679373271e-1) * t169 * t289 * t4598;
    (t5710, t5713, t5717, t5723, t5726, t5730)
}
