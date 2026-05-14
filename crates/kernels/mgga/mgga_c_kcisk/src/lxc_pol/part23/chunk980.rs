//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 980/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk980<F: Float>(t1342: F, t19861: F, t1339: F, t3754: F, t5606: F, t3769: F, t1440: F, t5967: F, t1341: F, t3785: F, t1411: F, t3512: F, t5612: F, t2152: F, t3777: F) -> (F, F, F, F, F, F, F) {
    let t19862 = t19861 * t1342;
    let t19863 = t1339 * t19862;
    let t19865 = t5606 * t3754;
    let t19866 = t1339 * t19865;
    let t19870 = t5606 * t3769;
    let t19871 = t1339 * t19870;
    let t19873 = t5967 * t1440;
    let t19874 = t1341 * t19873;
    let t19875 = t3785 * t19874;
    let t19876 = t1411 * t19875;
    let t19878 = t3512 * t5612;
    let t19879 = t1339 * t19878;
    let t19881 = t2152 * t3777;
    (t19863, t19866, t19871, t19873, t19876, t19879, t19881)
}
