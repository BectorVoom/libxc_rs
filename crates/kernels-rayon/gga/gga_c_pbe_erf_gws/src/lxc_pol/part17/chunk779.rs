//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 779/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk779(t1354: f64, t299: f64, t169: f64, t242: f64, t1343: f64, t700: f64, t1383: f64, t766: f64, t1355: f64, t770: f64, t289: f64, t4598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5708 = t299 * t1354;
    let t5710 = t169 * t5708 * t242;
    let t5713 = t169 * t1343 * t700;
    let t5717 = 0.15917832887339686635e0_f64 * t169 * t766 * t1383;
    let t5723 = t169 * t1355 * t700;
    let t5726 = t169 * t770 * t1383;
    let t5730 = 0.31835665774679373271e-1_f64 * t169 * t289 * t4598;
    (t5710, t5713, t5717, t5723, t5726, t5730)
}
