//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1083/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1083(t14464: f64, t3919: f64, t11535: f64, t14469: f64, t11475: f64, t14906: f64, t3931: f64, t14911: f64, t3972: f64, t11661: f64, t242: f64, t4826: f64, t8528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14928 = t3919 * t14464;
    let t14931 = t11535 * t14469;
    let t14934 = t11475 * t14906;
    let t14935 = t3931 * t14934;
    let t14938 = t3972 * t14911;
    let t14939 = t3931 * t14938;
    let t14942 = t11661 * t14906;
    let t14943 = t3931 * t14942;
    let t14947 = t242 * t8528 * t4826;
    (t14928, t14931, t14935, t14939, t14943, t14947)
}
