//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1150/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1150<F: Float>(t207: F, t40394: F, t40399: F, t786: F, t9580: F, t2566: F, t2570: F, t2588: F, t40341: F, t215: F, t39933: F, t40344: F, t795: F) -> (F, F, F, F, F, F) {
    let t41185 = F::cast_from(0.69444444444444444445e-4_f64) * t40394 * t207 * t40399;
    let t41189 = t9580 * t786;
    let t41196 = t2566 * t2570;
    let t41200 = F::cast_from(0.99537037037037037035e-1_f64) * t40341 * t2588;
    let t41209 = F::cast_from(0.14979423868312757201e0_f64) * t39933 * t207 * t215;
    let t41212 = F::cast_from(0.11265432098765432099e0_f64) * t40344 * t207 * t795;
    (t41185, t41189, t41196, t41200, t41209, t41212)
}
