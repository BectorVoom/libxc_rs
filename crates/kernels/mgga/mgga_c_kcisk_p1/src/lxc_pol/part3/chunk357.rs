//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 357/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk357<F: Float>(t604: F, t662: F, t695: F, t1060: F, t1775: F, t661: F, t657: F, t1689: F) -> (F, F, F, F, F, F, F) {
    let t659 = F::new(0.0) < t604;
    let t1776 = t662 * t695;
    let t1777 = t1776 * t1060;
    let t1778 = t1775 * t1777;
    let t1781 = t661 * t661;
    let t1782 = F::new(1.0) / t1781;
    let t1783 = t657 * t1782;
    let t1785 = piecewise3::<F>(t659, t1689, -t1689);
    (t1776, t1777, t1778, t1781, t1782, t1783, t1785)
}
