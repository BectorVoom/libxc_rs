//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 970/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk970(t14364: f64, t313: f64, t795: f64, t10914: f64, t10915: f64, t2639: f64, t43602: f64, t43604: f64, t45411: f64, t45415: f64, t45421: f64, t45426: f64, t45429: f64, t45432: f64, t45438: f64, t45440: f64, t45442: f64, t45451: f64, t45453: f64, t45454: f64, t45457: f64, t45458: f64, t45459: f64, t50130: f64, t6066: f64, t6111: f64) -> f64 {
    let t50139 = t313 * t795 * t14364;
    let t50149 = -0.10725146985555128001e1_f64 * t50139 * t2639 + 0.85801175884441024008e1_f64 * t6111 * t6066 * t50130 - 0.42900587942220512004e1_f64 * t10914 * t10915 * t50130 - t45411 + t45415 + t43602 - t43604 - 0.44688112439813033337e-1_f64 * t45421 + t45426 - t45429 - t45432 - t45438 + t45440 - t45442 + t45451 + t45453 + t45454 + t45457 - t45458 + t45459;
    t50149
}
