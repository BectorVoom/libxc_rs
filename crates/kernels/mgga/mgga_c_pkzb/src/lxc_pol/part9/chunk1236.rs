//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1236/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1236<F: Float>(t178: F, t18152: F, t5953: F, t5719: F, t2899: F, t7728: F, t774: F, t7732: F, t7736: F, t7738: F, t7742: F, t7744: F) -> (F, F, F, F, F, F, F) {
    let t21603 = t18152 * t178;
    let t21604 = t5953 * t21603;
    let t21607 = t5719 * t21603;
    let t21611 = t2899 * t774 * t7728;
    let t21614 = t2899 * t774 * t7732;
    let t21617 = t7736 * t774 * t7738;
    let t21620 = t7742 * t774 * t7744;
    (t21603, t21604, t21607, t21611, t21614, t21617, t21620)
}
