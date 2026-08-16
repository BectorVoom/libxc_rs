//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1193/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1193(t11864: f64, t11866: f64, t13788: f64, t2064: f64, t439: f64, t477: f64, t822: f64, t11868: f64, t1385: f64, t1868: f64, t2010: f64, t5168: f64, t6372: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15747 = 128.0_f64 / 405.0_f64 * t11864;
    let t15748 = 8.0_f64 / 405.0_f64 * t11866;
    let t15753 = 4.0_f64 / 5.0_f64 * t439 * t13788 * t822 * t477 * t2064;
    let t15754 = 2.0_f64 / 45.0_f64 * t11868;
    let t15758 = 8.0_f64 / 45.0_f64 * t2010 * t1385 * t1868 * t2064;
    let t15760 = 8.0_f64 / 45.0_f64 * t5168 * t6372;
    (t15747, t15748, t15753, t15754, t15758, t15760)
}
