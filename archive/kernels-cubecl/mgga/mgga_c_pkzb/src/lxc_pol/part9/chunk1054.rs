//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1054/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1054<F: Float>(t5241: F, t6966: F, t1727: F, t5376: F, t1756: F, t5384: F, t5370: F, t1511: F, t5331: F, t1613: F, t4952: F, t542: F, t555: F) -> (F, F, F, F, F, F) {
    let t16453 = t6966 * t5241;
    let t16459 = t1727 * t5376;
    let t16467 = t5384 * t1756;
    let t16474 = t1727 * t5370;
    let t16476 = t1511 * t5331;
    let t16481 = F::cast_from(0.46785788981077169656e1_f64) * t555 * t1613 * t4952 * t542;
    (t16453, t16459, t16467, t16474, t16476, t16481)
}
