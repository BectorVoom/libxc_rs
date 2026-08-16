//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 754/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk754(t644: f64, t6887: f64, t647: f64, t2260: f64, t3912: f64, t256: f64, t5236: f64, t5240: f64, t5284: f64, t5304: f64, t6858: f64, t6860: f64, t6862: f64, t6864: f64, t6869: f64, t6871: f64, t6873: f64, t6877: f64, t6879: f64, t6881: f64, t6885: f64) -> (f64, f64, f64, f64) {
    let t6888 = t6887 * t644;
    let t6889 = t6888 * t647;
    let t6892 = (2e-21_f64 as f64) * t2260 * t3912;
    let t6893 = -t6858 - t6860 + t6862 + t6864 + t6869 - t6871 - t6873 - t6877 + t6879 + t6881 * t256 / 3.0_f64 + t6885 / 3.0_f64 + 0.06077777777777778_f64 * t6889 + t6892 - t5236 + t5240 + t5284 - t5304;
    (t6888, t6889, t6892, t6893)
}
