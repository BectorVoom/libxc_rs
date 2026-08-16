//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2632/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2632(t11638: f64, t11868: f64, t11877: f64, t11881: f64, t1235: f64, t1244: f64, t1246: f64, t14985: f64, t14989: f64, t15000: f64, t15239: f64, t1734: f64, t1755: f64, t3590: f64, t3604: f64, t3610: f64, t3612: f64, t3624: f64, t3625: f64, t470: f64, t493: f64, t5011: f64, t5068: f64, t5072: f64, t5073: f64, t5079: f64, t52500: f64, t53529: f64) -> f64 {
    let t53538 = 2.0_f64 * t11638 * t1755 * t3610 * t3612 + t11868 * t1244 * t1246 * t1734 + 3.0_f64 * t1235 * t1244 * t1246 * t15239 + 3.0_f64 * t1244 * t1246 * t3590 * t5011 + 18.0_f64 * t11881 * t15000 * t5072 + 6.0_f64 * t14985 * t3610 * t5068 - 3.0_f64 * t14985 * t3624 * t5079 - 3.0_f64 * t3624 * t3625 * t52500 + t470 * t493 * t53529 + 3.0_f64 * t11877 * t5073 + 6.0_f64 * t14989 * t3604;
    t53538
}
