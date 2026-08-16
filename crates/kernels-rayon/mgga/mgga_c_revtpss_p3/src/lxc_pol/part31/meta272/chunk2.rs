//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1222/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1222(t1583: f64, t30: f64, t1468: f64, t1940: f64, t1963: f64, t2403: f64, t7091: f64, t7750: f64, t7783: f64, t1659: f64, t1972: f64, t1656: f64, t1665: f64, t1671: f64, t1675: f64, t375: f64, t7110: f64, t7111: f64, t7117: f64, t7122: f64, t7130: f64, t7132: f64) -> (f64, f64, f64, f64) {
    let t7787 = t30 * t1583;
    let t7794 = 3.0_f64 / 2.0_f64 * t2403 * t7750 + t1940 * t7783 * t30 / 2.0_f64 - t1940 * t7091 * t7787 / 2.0_f64 + t1940 * t1963 * t1468 / 2.0_f64;
    let t7801 = t1659 * t1972;
    let t7810 = t7110 + t7111 * t1656 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t7801 * t375 - 0.42874018118069736972e-3_f64 * t7117 * t1665 + 0.42874018118069736972e-3_f64 * t7122 * t1671 + t7130 + 0.28582678745379824648e-3_f64 * t7132 * t1675;
    (t7787, t7794, t7801, t7810)
}
