//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1158/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1158<F: Float>(t14686: F, t3437: F, t10753: F, t5073: F, t1166: F, t5185: F, t3460: F, t5083: F, t13265: F, t3338: F, t5046: F, t10787: F, t5091: F) -> (F, F, F, F, F, F) {
    let t14687 = t3437 * t14686;
    let t14689 = t10753 * t5073;
    let t14691 = t1166 * t5185;
    let t14693 = t5083 * t3460;
    let t14695 = t3338 * t13265;
    let t14696 = t5046 * t14695;
    let t14698 = t10787 * t5091;
    (t14687, t14689, t14691, t14693, t14696, t14698)
}
