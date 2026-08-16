//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 747/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk747(t10478: f64, t3128: f64, t10472: f64, t1015: f64, t10277: f64, t2978: f64, t10213: f64, t10216: f64, t2775: f64, t283: f64, t61: f64, t2770: f64, t976: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10930 = t2978 * t10277;
    let t10942 = t10213 * t10216;
    let t10969 = 1.0_f64 / t283 / t2775;
    let t10970 = t61 * t10969;
    let t10996 = t976 * t2770;
    (t10876, t10883, t10930, t10942, t10970, t10996)
}
