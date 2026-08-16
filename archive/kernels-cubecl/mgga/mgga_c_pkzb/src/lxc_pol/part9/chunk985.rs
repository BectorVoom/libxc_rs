//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 985/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk985<F: Float>(t7737: F, t7832: F, t1123: F, t785: F, t2019: F, t2916: F, t306: F, t2968: F, t5718: F, t7743: F, t2036: F, t133: F, t7575: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7833 = t7832 * t7737;
    let t7836 = t785 * t1123;
    let t7837 = t2019 * t7836;
    let t7840 = t306 * t2916;
    let t7841 = t2019 * t7840;
    let t7844 = t5718 * t2968;
    let t7845 = t7832 * t7743;
    let t7854 = t2036 * t7836;
    let t7857 = t7575 * t133;
    (t7833, t7836, t7837, t7840, t7841, t7844, t7845, t7854, t7857)
}
