//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2637/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2637(t5052: f64, t6224: f64, t11881: f64, t1215: f64, t1244: f64, t1246: f64, t19165: f64, t19201: f64, t22340: f64, t22348: f64, t22358: f64, t22364: f64, t22368: f64, t22386: f64, t3610: f64, t3624: f64, t3625: f64, t44698: f64, t44701: f64, t44753: f64, t44754: f64, t45326: f64, t491: f64, t5068: f64, t5072: f64, t5084: f64, t6218: f64, t72217: f64) -> (f64, f64) {
    let t73720 = t5052 * t6224;
    let t73736 = -36.0_f64 * t1215 * t22348 * t44698 * t44701 + 14.0_f64 * t1215 * t22348 * t44753 * t44754 + t1244 * t1246 * t491 * t72217 + 3.0_f64 * t1244 * t1246 * t5052 * t6218 + 18.0_f64 * t11881 * t19165 * t22364 + 6.0_f64 * t22340 * t3610 * t5068 + 6.0_f64 * t22368 * t3610 * t5072 + 2.0_f64 * t22386 * t3610 * t5068 - 3.0_f64 * t3624 * t3625 * t73720 + 3.0_f64 * t19201 * t5084 + 6.0_f64 * t22358 * t45326;
    (t73720, t73736)
}
