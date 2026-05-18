//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 959/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk959<F: Float>(t2714: F, t3401: F, t1020: F, t9116: F, t8778: F, t3363: F, t972: F) -> (F, F, F, F) {
    let t10506 = t2714 * t3401;
    let t10509 = t9116 * t1020;
    let t10512 = F::new(3.0) * t8778;
    let t10513 = t3363 * t972;
    (t10506, t10509, t10512, t10513)
}
