//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1240/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1240<F: Float>(t2049: F, t2768: F, t2061: F, t6044: F, t759: F, t955: F, t22625: F, t7877: F, t2461: F, t2056: F, t6028: F, t6001: F, t7872: F, t22595: F, t2823: F, t22602: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26926 = t2768 * t2049;
    let t26927 = t2061 * t26926;
    let t26928 = 0.2025780996e0 * t26927;
    let t26932 = t759 * t955 * t6044;
    let t26938 = t7877 * t22625;
    let t26944 = t759 * t2461 * t2049;
    let t26945 = 0.857292e-1 * t26944;
    let t26947 = t2768 * t2056;
    let t26948 = t6028 * t26947;
    let t26949 = 0.4051561992e0 * t26948;
    let t26960 = t7872 * t6001;
    let t26961 = 0.2025780996e0 * t26960;
    let t26963 = t2823 * t22595;
    let t26964 = 0.2025780996e0 * t26963;
    let t26965 = t2823 * t22602;
    (t26926, t26928, t26932, t26938, t26945, t26947, t26949, t26961, t26964, t26965)
}
