//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3113/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3113<F: Float>(t13032: F, t17528: F, t247: F, t44545: F, t5230: F, t5384: F, t12984: F, t5327: F, t12995: F, t17438: F, t1214: F, t1222: F, t1235: F, t1250: F, t12629: F, t12866: F, t17280: F, t17475: F, t17484: F, t17534: F, t17649: F, t1808: F, t20945: F, t2251: F, t3362: F, t3613: F, t3667: F, t371: F, t3718: F, t372: F, t3720: F, t44427: F, t44568: F, t44609: F, t44664: F, t471: F, t482: F, t5351: F, t5405: F, t56179: F, t56376: F, t57200: F, t57209: F, t57212: F, t57214: F, t57223: F, t57227: F) -> F {
    let t57229 = t13032 * t17528;
    let t57241 = t5384 * t247 * t44545 * t5230;
    let t57242 = F::cast_from(0.28582678745379824648e-3_f64) * t57241;
    let t57250 = t5327 * t12984;
    let t57251 = F::cast_from(0.14291339372689912324e-3_f64) * t57250;
    let t57252 = t17438 * t12995;
    let t57254 = F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17649 * t17534 * t5405 + F::cast_from(0.64311027177104605458e-3_f64) * t44664 * t17484 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t3720 * t57200 * t1250 - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1222 * t17475 * t56179 + t57209 / F::cast_from(108.0_f64) + t57212 / F::cast_from(216.0_f64) - t57214 / F::cast_from(81.0_f64) - F::cast_from(0.12862205435420921092e-2_f64) * t44609 * t3720 * t5351 * t471 * t12629 + t57223 - F::cast_from(0.14291339372689912324e-3_f64) * t44568 * t1808 - F::cast_from(0.11433071498151929859e-2_f64) * t57227 + F::cast_from(0.34299214494455789577e-2_f64) * t57229 * t3613 + F::cast_from(0.47637797908966374413e-3_f64) * t44427 - F::cast_from(0.7145669686344956162e-3_f64) * t12866 * t20945 * t1250 * t1214 * t3362 * t2251 - t57242 - F::cast_from(0.64311027177104605458e-3_f64) * t3667 * t17280 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t371 * t372 * t482 * t56376 + t57251 - F::cast_from(0.45732285992607719436e-2_f64) * t57252;
    t57254
}
