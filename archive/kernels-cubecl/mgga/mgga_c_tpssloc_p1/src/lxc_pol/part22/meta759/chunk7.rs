//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2556/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2556<F: Float>(t11310: F, t11361: F, t11365: F, t1148: F, t1155: F, t1156: F, t1683: F, t21907: F, t21939: F, t21942: F, t21947: F, t3371: F, t44205: F, t44220: F, t4857: F, t51371: F, t51677: F, t6069: F, t6088: F, t64254: F, t71530: F, t71543: F, t71545: F, t71547: F, t71655: F, t71657: F) -> F {
    let t71664 = F::cast_from(0.51947577317044391276e2_f64) * t51371 * t6088 - F::cast_from(0.10389515463408878255e3_f64) * t44205 * t21907 + F::cast_from(0.5848223622634646207e0_f64) * t3371 * t21939 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t71530 * t1156 + F::cast_from(0.10254018858216406658e4_f64) * t44220 * t21942 - F::cast_from(0.35089341735807877242e1_f64) * t51677 * t6069 + F::cast_from(0.35089341735807877242e1_f64) * t11361 * t21947 + F::cast_from(3.0_f64) * t64254 * t1683 - t71543 + t71545 - t71547 - t71655 - t71657 + F::cast_from(0.6233709278045326953e3_f64) * t11310 * t21907 * t1155 - F::cast_from(0.31168546390226634765e3_f64) * t11365 * t6088 * t4857;
    t71664
}
