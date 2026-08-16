//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1018/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1018(t1695: f64, t988: f64, t1678: f64, t7150: f64, t1651: f64, t11239: f64, t1096: f64, t1646: f64, t33: f64, t41154: f64, t1518: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99638 = t1695 * t988;
    let t99914 = t7150 * t1678;
    let t99970 = t1651 * t988;
    let t100533 = t1678 * t11239;
    let t100743 = t1646 * t1096;
    let t100981 = t41154 * t33;
    let t105823 = t1518 * t1936;
    (t99638, t99914, t99970, t100533, t100743, t100981, t105823)
}
