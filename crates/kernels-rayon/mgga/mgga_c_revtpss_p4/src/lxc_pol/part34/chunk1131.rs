//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1131/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1131(t342: f64, t7810: f64, t1678: f64, t3140: f64, t1078: f64, t1982: f64, t11239: f64, t1035: f64, t1983: f64, t1668: f64, t1976: f64, t3153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27616 = t342 * t7810;
    let t27619 = t1678 * t3140;
    let t27621 = t1982 * t27619 * t1078;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    let t27641 = t1976 * t1668;
    let t27642 = t27641 * t3153;
    (t27616, t27621, t27638, t27639, t27640, t27642)
}
