//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1249/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1249<F: Float>(t2096: F, t7692: F, t287: F, t5913: F, t17848: F, t2104: F, t7641: F, t17867: F, t2932: F, t7607: F, t7784: F, t2945: F, t2947: F, t5939: F) -> (F, F, F, F, F, F) {
    let t21841 = t2096 * t7692;
    let t21843 = t5913 * t287;
    let t21852 = t2104 * t17848 * t7641;
    let t21862 = t2104 * t17867 * t2932;
    let t21863 = F::new(0.28582678745379824648e-3) * t21862;
    let t21867 = t7607 * t7784;
    let t21870 = t2945 * t5939 * t2947;
    (t21841, t21843, t21852, t21863, t21867, t21870)
}
