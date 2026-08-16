//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 997/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk997(t10714: f64, t3564: f64, t177: f64, t3590: f64, t737: f64, t2112: f64, t3569: f64, t1992: f64, t3565: f64, t162: f64, t8082: f64, t3566: f64) -> (f64, f64, f64, f64, f64) {
    let t10716 = 24.0_f64 * t3564 * t10714;
    let t10717 = t3590 * t177;
    let t10719 = 0.11696447245269292414e1_f64 * t10717 * t737;
    let t10721 = 8.0_f64 * t2112 * t3569;
    let t10722 = t3565 * t1992;
    let t10724 = 12.0_f64 * t3564 * t10722;
    let t10725 = t8082 * t162;
    let t10727 = 24.0_f64 * t10725 * t3566;
    (t10716, t10719, t10721, t10724, t10727)
}
