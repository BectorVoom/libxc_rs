//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 422/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk422<F: Float>(t1667: F, t550: F, t46: F, t512: F, t552: F, t637: F) -> (F, F, F, F, F) {
    let t1669 = F::new(0.24415263074675393405e-3) * t550 * t1667;
    let t1670 = t512 * t46;
    let t1671 = t1670 * t552;
    let t1672 = F::new(0.36622894612013090108e-3) * t1671;
    let t1673 = t637 * t637;
    (t1669, t1670, t1671, t1672, t1673)
}
