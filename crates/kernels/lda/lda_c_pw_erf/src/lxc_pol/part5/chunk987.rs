//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 987/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk987<F: Float>(t1949: F, t2478: F, t3974: F, t4574: F, t1944: F, t5165: F, t12314: F, t6725: F, t15607: F, t6730: F, t6734: F, t6737: F, t15672: F, t20826: F, t20829: F, t20832: F, t20835: F, t20837: F, t20840: F) -> (F, F, F, F, F, F, F, F) {
    let t20844 = 16.0 / 15.0 * t3974 * t4574 * t2478 * t1949;
    let t20848 = 8.0 / 9.0 * t3974 * t5165 * t2478 * t1944;
    let t20850 = 16.0 / 15.0 * t12314 * t6725;
    let t20852 = 16.0 / 15.0 * t15607 * t6730;
    let t20854 = 16.0 / 15.0 * t15607 * t6734;
    let t20856 = 8.0 / 9.0 * t15607 * t6737;
    let t20857 = 16.0 / 27.0 * t15672;
    let t20858 = t20826 - t20829 + t20832 - t20835 - t20837 - t20840 - t20844 + t20848 - t20850 + t20852 + t20854 - t20856 + t20857;
    (t20844, t20848, t20850, t20852, t20854, t20856, t20857, t20858)
}
