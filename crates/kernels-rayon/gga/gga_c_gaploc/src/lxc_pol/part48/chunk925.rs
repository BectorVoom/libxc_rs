//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 925/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk925(t45712: f64, t11845: f64, t2628: f64, t10811: f64, t10967: f64, t1457: f64, t1991: f64, t2103: f64, t41295: f64, t41299: f64, t41312: f64, t41316: f64, t43904: f64, t44888: f64, t44967: f64, t45066: f64, t45678: f64, t45680: f64, t45684: f64, t45688: f64, t45690: f64, t45693: f64, t45700: f64, t45703: f64, t45711: f64, t590: f64, t739: f64) -> f64 {
    let t45713 = 0.14896037479937677779e-1_f64 * t45712;
    let t45716 = t11845 * t2628;
    let t45717 = 0.29792074959875355558e-1_f64 * t45716;
    let t45718 = 0.20449560508757733161e1_f64 * t1991 * t739 * t44888 * t590 - 0.10427226235956374446e0_f64 * t45678 + t45680 + t45684 + t45688 - t45690 - t45693 - 0.63904876589867916126e-1_f64 * t41295 - 0.63904876589867916126e-1_f64 * t41299 + 0.63904876589867916126e-1_f64 * t41312 + 0.63904876589867916126e-1_f64 * t41316 + t45700 + t45703 + 0.14300195980740170668e1_f64 * t2103 * t1457 * t44967 + 0.14300195980740170668e1_f64 * t2103 * t1457 * t45066 - 0.1022478025437886658e1_f64 * t43904 + t45711 + t45713 + 0.85801175884441024008e1_f64 * t10811 * t10967 - t45717;
    t45718
}
