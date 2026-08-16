//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3113/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3113(t13032: f64, t17528: f64, t247: f64, t44545: f64, t5230: f64, t5384: f64, t12984: f64, t5327: f64, t12995: f64, t17438: f64, t1214: f64, t1222: f64, t1235: f64, t1250: f64, t12629: f64, t12866: f64, t17280: f64, t17475: f64, t17484: f64, t17534: f64, t17649: f64, t1808: f64, t20945: f64, t2251: f64, t3362: f64, t3613: f64, t3667: f64, t371: f64, t3718: f64, t372: f64, t3720: f64, t44427: f64, t44568: f64, t44609: f64, t44664: f64, t471: f64, t482: f64, t5351: f64, t5405: f64, t56179: f64, t56376: f64, t57200: f64, t57209: f64, t57212: f64, t57214: f64, t57223: f64, t57227: f64) -> f64 {
    let t57229 = t13032 * t17528;
    let t57241 = t5384 * t247 * t44545 * t5230;
    let t57242 = 0.28582678745379824648e-3_f64 * t57241;
    let t57250 = t5327 * t12984;
    let t57251 = 0.14291339372689912324e-3_f64 * t57250;
    let t57252 = t17438 * t12995;
    let t57254 = 0.85748036236139473944e-3_f64 * t12866 * t17649 * t17534 * t5405 + 0.64311027177104605458e-3_f64 * t44664 * t17484 - 0.21437009059034868486e-3_f64 * t3718 * t3720 * t57200 * t1250 - 7.0_f64 / 54.0_f64 * t1222 * t17475 * t56179 + t57209 / 108.0_f64 + t57212 / 216.0_f64 - t57214 / 81.0_f64 - 0.12862205435420921092e-2_f64 * t44609 * t3720 * t5351 * t471 * t12629 + t57223 - 0.14291339372689912324e-3_f64 * t44568 * t1808 - 0.11433071498151929859e-2_f64 * t57227 + 0.34299214494455789577e-2_f64 * t57229 * t3613 + 0.47637797908966374413e-3_f64 * t44427 - 0.7145669686344956162e-3_f64 * t12866 * t20945 * t1250 * t1214 * t3362 * t2251 - t57242 - 0.64311027177104605458e-3_f64 * t3667 * t17280 - 0.21437009059034868486e-3_f64 * t1235 * t371 * t372 * t482 * t56376 + t57251 - 0.45732285992607719436e-2_f64 * t57252;
    t57254
}
