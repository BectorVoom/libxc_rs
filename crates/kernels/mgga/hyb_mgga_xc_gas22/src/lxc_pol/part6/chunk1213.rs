//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1213/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1213<F: Float>(t21862: F, t33: F, t34: F, t3025: F, t1189: F, t3023: F, t1897: F, t7978: F, t1211: F, t6092: F, t8061: F, t81: F) -> (F, F, F, F, F, F) {
    let t23340 = t33 * t34 * t21862;
    let t23341 = t23340 * t3025;
    let t23351 = t3023 * t1189;
    let t23355 = t7978 * t1897;
    let t23453 = t6092 * t1211;
    let t23488 = t81 * t8061;
    (t23340, t23341, t23351, t23355, t23453, t23488)
}
