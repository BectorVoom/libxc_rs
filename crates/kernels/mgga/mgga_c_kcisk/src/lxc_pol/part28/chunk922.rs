//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 922/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk922<F: Float>(t1797: F, t918: F, t1248: F, t4889: F, t6764: F, t10999: F, t6759: F, t17402: F, t17399: F, t1774: F, t3117: F, t2404: F, t4857: F, t1705: F, t7088: F, t4908: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17410 = t918 * t1797;
    let t17423 = t1248 * t4889 * t6764;
    let t17424 = 0.43816888888888888888e0 * t17423;
    let t17426 = t1248 * t10999 * t6759;
    let t17453 = 4.0 / 27.0 * t17402;
    let t17454 = 4.0 / 9.0 * t17399;
    let t17480 = t3117 * t1774;
    let t17505 = 0.41203703703703703704e-2 * t17402;
    let t17506 = 0.12361111111111111111e-1 * t17399;
    let t17520 = t2404 * t4857;
    let t17562 = t7088 * t1705;
    let t17567 = t2404 * t4908;
    (t17410, t17423, t17424, t17426, t17453, t17454, t17480, t17505, t17506, t17520, t17562, t17567)
}
