//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3745/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3745(t1715: f64, t3601: f64, t20816: f64, t3708: f64, t1121: f64, t1222: f64, t13053: f64, t17353: f64, t17448: f64, t17475: f64, t17640: f64, t17650: f64, t372: f64, t44521: f64, t44751: f64, t5277: f64, t5330: f64, t5335: f64, t57480: f64, t58868: f64, t58878: f64, t58882: f64, t58884: f64, t59066: f64, t59854: f64, t68265: f64, t68308: f64, t68345: f64) -> (f64, f64) {
    let t71200 = t1715 * t3601;
    let t71207 = t3708 * t20816;
    let t71231 = 0.17149607247227894789e-2_f64 * t59066 * t17353 * t13053 * t71200 + 0.57165357490759649296e-3_f64 * t58868 - 0.6351706387862183255e-4_f64 * t44751 + 0.28582678745379824648e-3_f64 * t71207 - 0.28582678745379824648e-3_f64 * t58878 - 0.11433071498151929859e-2_f64 * t44521 * t372 * t5277 * t1121 * t17650 - 0.85748036236139473944e-3_f64 * t59854 * t5330 * t5335 - 0.19055119163586549765e-3_f64 * t58882 - 0.57165357490759649296e-3_f64 * t58884 - 7.0_f64 / 54.0_f64 * t1222 * t17475 * t68345 - 7.0_f64 / 648.0_f64 * t1222 * t17475 * t68265 + 35.0_f64 / 972.0_f64 * t1222 * t57480 * t68308 - 0.28582678745379824648e-3_f64 * t17448 * t17640;
    (t71200, t71231)
}
