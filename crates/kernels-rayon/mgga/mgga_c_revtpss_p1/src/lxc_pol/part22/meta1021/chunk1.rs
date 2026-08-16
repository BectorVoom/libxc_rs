//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3550/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3550(t1063: f64, t11986: f64, t247: f64, t6096: f64, t1045: f64, t15785: f64, t19572: f64, t3115: f64, t3117: f64, t3120: f64, t4892: f64, t4894: f64, t55293: f64, t55320: f64, t55325: f64, t55328: f64, t55361: f64, t55367: f64, t66565: f64, t67545: f64, t67551: f64, t67560: f64, t67568: f64, t67571: f64) -> f64 {
    let t67575 = t1063 * t247 * t11986 * t6096;
    let t67578 = 0.19055119163586549765e-3_f64 * t55293 - 0.42874018118069736972e-3_f64 * t3115 * t3117 * t67545 * t1045 - 0.42874018118069736972e-3_f64 * t67551 * t3120 + 0.42874018118069736972e-3_f64 * t4892 * t3117 * t19572 * t15785 - 0.28582678745379824648e-3_f64 * t55320 + 0.11433071498151929859e-2_f64 * t67560 + 0.85748036236139473944e-3_f64 * t4892 * t3117 * t66565 * t4894 - 0.17149607247227894789e-2_f64 * t55325 + 0.28582678745379824648e-3_f64 * t55328 + 0.28582678745379824648e-3_f64 * t67568 + 0.11433071498151929859e-2_f64 * t55361 + 0.20325460441158986416e-2_f64 * t67571 + 0.6351706387862183255e-4_f64 * t67575 + 0.11433071498151929859e-2_f64 * t55367;
    t67578
}
