//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1123/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1123(t15607: f64, t6730: f64, t6734: f64, t6737: f64, t15672: f64, t20826: f64, t20829: f64, t20832: f64, t20835: f64, t20837: f64, t20840: f64, t20844: f64, t20848: f64, t20850: f64) -> (f64, f64, f64, f64, f64) {
    let t20852 = 16.0_f64 / 15.0_f64 * t15607 * t6730;
    let t20854 = 16.0_f64 / 15.0_f64 * t15607 * t6734;
    let t20856 = 8.0_f64 / 9.0_f64 * t15607 * t6737;
    let t20857 = 16.0_f64 / 27.0_f64 * t15672;
    let t20858 = t20826 - t20829 + t20832 - t20835 - t20837 - t20840 - t20844 + t20848 - t20850 + t20852 + t20854 - t20856 + t20857;
    (t20852, t20854, t20856, t20857, t20858)
}
