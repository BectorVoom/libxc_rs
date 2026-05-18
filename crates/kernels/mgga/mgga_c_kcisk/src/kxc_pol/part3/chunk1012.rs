//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1012/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1012<F: Float>(t3973: F, t4387: F, t1580: F, t13917: F, t4392: F, t12924: F, t1581: F, t1312: F, t12829: F, t539: F, t12830: F, t13894: F) -> (F, F, F, F) {
    let t14921 = t3973 * t4387;
    let t14922 = t1580 * t14921;
    let t14924 = t13917 * t4392;
    let t14925 = t1580 * t14924;
    let t14929 = t1581 * t12924;
    let t14930 = t1312 * t14929;
    let t14935 = t539 * t12829;
    let t14936 = t14935 * t12830;
    let t14937 = t13894 * t14936;
    (t14922, t14925, t14930, t14937)
}
