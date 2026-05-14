//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1128/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1128<F: Float>(t25121: F, t55901: F, t14294: F, t16917: F, t123: F, t4561: F, t24468: F, t55912: F, t894: F, t897: F, t1235: F, t49244: F, t4565: F, t4768: F, t4961: F, t2669: F) -> (F, F, F, F, F, F, F, F, F) {
    let t56718 = t25121 * t55901;
    let t56722 = t14294 * t16917;
    let t56726 = t4561 * t123;
    let t56727 = t24468 * t56726;
    let t56732 = t894 * t897 * t55912;
    let t56735 = t49244 * t1235;
    let t56740 = t4768 * t4565;
    let t56744 = t4961 * t4961;
    let t56745 = t2669 * t56744;
    (t56718, t56722, t56726, t56727, t56732, t56735, t56740, t56744, t56745)
}
