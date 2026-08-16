//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 814/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk814(t3739: f64, t8079: f64, t1413: f64, t8161: f64, t8074: f64, t8130: f64, t960: f64, t8133: f64, t965: f64, t8136: f64, t8123: f64, t970: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25327 = t3739 * t8079;
    let t25350 = t8161 * t1413;
    let t25351 = t25350 * sigma0;
    let t25376 = t3739 * t8074;
    let t25425 = t960 * t8130;
    let t25427 = t965 * t8133;
    let t25429 = t965 * t8136;
    let t25485 = t970 * t8123;
    (t25327, t25350, t25351, t25376, t25425, t25427, t25429, t25485)
}
