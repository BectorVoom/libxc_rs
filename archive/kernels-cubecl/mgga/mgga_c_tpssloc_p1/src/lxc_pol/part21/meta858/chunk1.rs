//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3116/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3116<F: Float>(t18918: F, t3411: F, t1703: F, t51807: F, t14858: F, t4879: F, t15036: F, t4869: F, t1155: F, t4857: F, t4861: F, t51848: F) -> (F, F, F, F, F, F) {
    let t64475 = F::cast_from(0.46785788981077169656e1_f64) * t3411 * t18918;
    let t64477 = F::cast_from(0.11696447245269292414e1_f64) * t51807 * t1703;
    let t64479 = F::cast_from(0.23392894490538584828e1_f64) * t14858 * t4879;
    let t64481 = F::cast_from(0.70178683471615754484e1_f64) * t4869 * t15036;
    let t64482 = t1155 * t4857;
    let t64485 = F::cast_from(0.4155806185363551302e3_f64) * t51848 * t4861 * t64482;
    (t64475, t64477, t64479, t64481, t64482, t64485)
}
