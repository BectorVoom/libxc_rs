//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 651/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk651(t1446: f64, t1462: f64, t1449: f64, t1453: f64, t519: f64, t1458: f64, t9: f64, t1461: f64, t2961: f64, t523: f64, t522: f64, t1251: f64, t187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3879 = 4.0_f64 / 9.0_f64 * t1446 * t1462;
    let t3880 = t1449 * t1453;
    let t3881 = t519 * t3880;
    let t3882 = 8.0_f64 / 45.0_f64 * t3881;
    let t3883 = t9 * t1458;
    let t3884 = t3883 * t1461;
    let t3885 = t519 * t3884;
    let t3886 = 8.0_f64 / 27.0_f64 * t3885;
    let t3887 = t523 * t2961;
    let t3888 = t522 * t3887;
    let t3890 = 4.0_f64 / 45.0_f64 * t519 * t3888;
    let t3892 = 1.0_f64 / t187 / t1251;
    (t3879, t3880, t3881, t3882, t3883, t3884, t3885, t3886, t3887, t3888, t3890, t3892)
}
