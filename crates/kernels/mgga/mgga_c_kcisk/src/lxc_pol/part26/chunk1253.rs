//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1253/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1253<F: Float>(t111228: F, t31902: F, t15202: F, t2933: F, t31896: F, t927: F, t31895: F, t113: F, t15208: F, t31894: F, t2935: F, t3058: F, t932: F, t3050: F, t397: F, t933: F) -> (F, F, F, F, F, F, F, F, F) {
    let t111229 = t31902 * t111228;
    let t111233 = t31896 * t15202 * t2933 * t927;
    let t111234 = t31895 * t111233;
    let t111236 = t15208 * t113;
    let t111237 = t111236 * t31894;
    let t111238 = t111237 * t111233;
    let t111242 = t31896 * t2935 * t3058 * t932;
    let t111243 = t31902 * t111242;
    let t111245 = t397 * t3050;
    let t111249 = t31902 * t111245 * t933 * t3058 * t927;
    (t111229, t111233, t111234, t111237, t111238, t111242, t111243, t111245, t111249)
}
