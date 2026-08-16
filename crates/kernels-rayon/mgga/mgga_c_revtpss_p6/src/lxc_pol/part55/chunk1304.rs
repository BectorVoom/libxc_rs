//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1304/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1304(t2118: f64, t8240: f64, t34848: f64, t571: f64, t34829: f64, t575: f64, t124429: f64, t124435: f64, t124438: f64, t131133: f64, t131134: f64, t131135: f64, t131148: f64, t131155: f64, t131159: f64, t131170: f64, t1456: f64, t1458: f64, t1464: f64, t1921: f64, t33317: f64, t34830: f64, t5808: f64, t7542: f64, t7560: f64, t8241: f64, t8249: f64, t8901: f64) -> f64 {
    let t131175 = t8240 * t2118;
    let t131177 = t571 * t34848;
    let t131178 = t34829 * t575;
    let t131182 = t1456 * t34848 + t124429 + t131133 + t131134 + t131135 + t1458 * (t131148 + t131155 + t131159 + t131170) + t34830 * t1464 + t124438 + t131175 + t8241 * t7560 + t124435 + t131177 + t131178 + t33317 * t1921 + t7542 * t8249 + t8901 * t5808;
    t131182
}
