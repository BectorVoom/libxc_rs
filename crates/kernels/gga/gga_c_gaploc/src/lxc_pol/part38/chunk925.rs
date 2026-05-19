//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 925/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk925<F: Float>(t45712: F, t11845: F, t2628: F, t10811: F, t10967: F, t1457: F, t1991: F, t2103: F, t41295: F, t41299: F, t41312: F, t41316: F, t43904: F, t44888: F, t44967: F, t45066: F, t45678: F, t45680: F, t45684: F, t45688: F, t45690: F, t45693: F, t45700: F, t45703: F, t45711: F, t590: F, t739: F) -> F {
    let t45713 = F::cast_from(0.14896037479937677779e-1_f64) * t45712;
    let t45716 = t11845 * t2628;
    let t45717 = F::cast_from(0.29792074959875355558e-1_f64) * t45716;
    let t45718 = F::cast_from(0.20449560508757733161e1_f64) * t1991 * t739 * t44888 * t590 - F::cast_from(0.10427226235956374446e0_f64) * t45678 + t45680 + t45684 + t45688 - t45690 - t45693 - F::cast_from(0.63904876589867916126e-1_f64) * t41295 - F::cast_from(0.63904876589867916126e-1_f64) * t41299 + F::cast_from(0.63904876589867916126e-1_f64) * t41312 + F::cast_from(0.63904876589867916126e-1_f64) * t41316 + t45700 + t45703 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t1457 * t44967 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t1457 * t45066 - F::cast_from(0.1022478025437886658e1_f64) * t43904 + t45711 + t45713 + F::cast_from(0.85801175884441024008e1_f64) * t10811 * t10967 - t45717;
    t45718
}
