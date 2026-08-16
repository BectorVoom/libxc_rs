//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 351/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk351(t1668: f64, t373: f64, t1045: f64, t1042: f64, t1066: f64, t1592: f64, t247: f64, t1009: f64, t1011: f64, t1025: f64, t1041: f64, t1060: f64, t1063: f64, t1656: f64, t1660: f64, t1665: f64, t375: f64) -> (f64, f64, f64, f64) {
    let t1669 = t373 * t1668;
    let t1670 = t1669 * t1045;
    let t1671 = t1042 * t1670;
    let t1674 = t1066 * t1592;
    let t1675 = t247 * t1674;
    let t1678 = t1009 + t1011 * t1656 / 288.0_f64 + 0.21437009059034868486e-3_f64 * t1660 * t375 - 0.21437009059034868486e-3_f64 * t1025 * t1665 + 0.21437009059034868486e-3_f64 * t1041 * t1671 + t1060 + 0.14291339372689912324e-3_f64 * t1063 * t1675;
    (t1670, t1671, t1675, t1678)
}
