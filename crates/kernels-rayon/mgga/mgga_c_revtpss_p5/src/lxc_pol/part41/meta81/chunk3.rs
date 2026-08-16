//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 480/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk480(t1678: f64, t225: f64, t385: f64, t1082: f64, t1651: f64, t1089: f64, t1668: f64, t378: f64, t380: f64, t1024: f64, t1087: f64, t1647: f64, t342: f64, t381: f64) -> (f64, f64, f64, f64, f64) {
    let t1679 = t1678 * t225;
    let t1680 = t1679 * t385;
    let t1685 = t1082 * t1651;
    let t1689 = t378 * t1668 * t1089;
    let t1692 = t380 * t1678;
    let t1695 = 0.65854491829355115987e0_f64 * t1647 * t381 - 0.65854491829355115987e0_f64 * t1024 * t1685 + 0.65854491829355115987e0_f64 * t1087 * t1689 + 0.65854491829355115987e0_f64 * t342 * t1692;
    (t1680, t1685, t1689, t1692, t1695)
}
