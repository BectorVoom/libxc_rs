//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 896/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk896<F: Float>(t1591: F, t4374: F, t4497: F, t6204: F, t3969: F, t4396: F, t4369: F, t1308: F, t1056: F, t4400: F, t1312: F, t3283: F, t13456: F, t4406: F, t4391: F, t3952: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t14873 = t4374 * t1591;
    let t14874 = t14873 * t4497;
    let t14875 = t6204 * t14874;
    let t14878 = t4396 * t3969;
    let t14885 = t4369 * sigma0;
    let t14886 = t14885 * t1308;
    let t14891 = t1056 * t4497;
    let t14892 = t4400 * t14891;
    let t14893 = t1312 * t14892;
    let t14896 = t3283 * t1591;
    let t14897 = t4400 * t14896;
    let t14898 = t1312 * t14897;
    let t14901 = t4406 * t13456;
    let t14902 = t1312 * t14901;
    let t14905 = t4391 * t13456;
    let t14906 = t3952 * t14905;
    (t14875, t14878, t14886, t14893, t14898, t14902, t14906)
}
