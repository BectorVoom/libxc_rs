//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 916/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk916(t2737: f64, t8507: f64, t2798: f64, t2782: f64, t2762: f64, t774: f64, t126: f64, t2761: f64, t242: f64, t2460: f64, t967: f64, t2464: f64, t277: f64) -> (f64, f64, f64, f64, f64) {
    let t8508 = t2737 * t8507;
    let t8509 = t2798 * t8508;
    let t8514 = t2782 * t8508;
    let t8523 = t774 * t2762;
    let t8528 = t126 * t2761;
    let t8530 = t242 * t8528 * t2460;
    let t8531 = t967 * t8530;
    let t8539 = 1.0_f64 / t277 / t2464;
    (t8509, t8514, t8523, t8531, t8539)
}
