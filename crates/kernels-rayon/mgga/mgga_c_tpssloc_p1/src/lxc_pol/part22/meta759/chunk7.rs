//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2556/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2556(t11310: f64, t11361: f64, t11365: f64, t1148: f64, t1155: f64, t1156: f64, t1683: f64, t21907: f64, t21939: f64, t21942: f64, t21947: f64, t3371: f64, t44205: f64, t44220: f64, t4857: f64, t51371: f64, t51677: f64, t6069: f64, t6088: f64, t64254: f64, t71530: f64, t71543: f64, t71545: f64, t71547: f64, t71655: f64, t71657: f64) -> f64 {
    let t71664 = 0.51947577317044391276e2_f64 * t51371 * t6088 - 0.10389515463408878255e3_f64 * t44205 * t21907 + 0.5848223622634646207e0_f64 * t3371 * t21939 + 0.5848223622634646207e0_f64 * t1148 * t71530 * t1156 + 0.10254018858216406658e4_f64 * t44220 * t21942 - 0.35089341735807877242e1_f64 * t51677 * t6069 + 0.35089341735807877242e1_f64 * t11361 * t21947 + 3.0_f64 * t64254 * t1683 - t71543 + t71545 - t71547 - t71655 - t71657 + 0.6233709278045326953e3_f64 * t11310 * t21907 * t1155 - 0.31168546390226634765e3_f64 * t11365 * t6088 * t4857;
    t71664
}
