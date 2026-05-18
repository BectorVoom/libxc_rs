//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 970/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk970<F: Float>(t14364: F, t313: F, t795: F, t10914: F, t10915: F, t2639: F, t43602: F, t43604: F, t45411: F, t45415: F, t45421: F, t45426: F, t45429: F, t45432: F, t45438: F, t45440: F, t45442: F, t45451: F, t45453: F, t45454: F, t45457: F, t45458: F, t45459: F, t50130: F, t6066: F, t6111: F) -> F {
    let t50139 = t313 * t795 * t14364;
    let t50149 = -F::new(0.10725146985555128001e1) * t50139 * t2639 + F::new(0.85801175884441024008e1) * t6111 * t6066 * t50130 - F::new(0.42900587942220512004e1) * t10914 * t10915 * t50130 - t45411 + t45415 + t43602 - t43604 - F::new(0.44688112439813033337e-1) * t45421 + t45426 - t45429 - t45432 - t45438 + t45440 - t45442 + t45451 + t45453 + t45454 + t45457 - t45458 + t45459;
    t50149
}
