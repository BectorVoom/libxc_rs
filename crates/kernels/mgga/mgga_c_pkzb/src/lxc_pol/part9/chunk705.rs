//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 705/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk705<F: Float>(t195: F, t4859: F, t1469: F, t642: F, t1821: F, t462: F, t1499: F, t466: F, t1531: F, t1508: F, t49: F, t512: F) -> (F, F, F, F, F, F, F, F) {
    let t4860 = t4859 * t195;
    let t4861 = t1469 * t642;
    let t4862 = F::new(3.0) * t4861;
    let t4863 = t462 * t1821;
    let t4864 = F::new(3.0) * t4863;
    let t4865 = t466 * t1499;
    let t4867 = F::new(0.16265371950452609763e-1) * t1531 * t4865;
    let t4868 = t466 * t1508;
    let t4870 = F::new(0.48159733137676571078e0) * t1531 * t4868;
    let t4871 = t512 * t49;
    (t4860, t4862, t4864, t4865, t4867, t4868, t4870, t4871)
}
