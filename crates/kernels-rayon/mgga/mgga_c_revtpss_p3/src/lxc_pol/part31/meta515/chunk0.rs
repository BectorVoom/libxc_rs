//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1868/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1868(t1089: f64, t1668: f64, t25681: f64, t4866: f64, t7168: f64, t7828: f64, t988: f64, t7160: f64, t1078: f64, t11239: f64, t1035: f64, t1983: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27627 = t25681 * t1668 * t1089;
    let t27631 = t7168 * t4866 * t1089;
    let t27634 = t7828 * t988;
    let t27635 = t7160 * t27634;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    (t27627, t27631, t27635, t27638, t27639, t27640)
}
