//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 779/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk779(t101: f64, t7213: f64, t2363: f64, t473: f64, t483: f64, t485: f64, t4153: f64, t4156: f64, t4160: f64, t4163: f64, t4165: f64, t4168: f64, t4172: f64, t4250: f64, t4258: f64, t5440: f64, t5442: f64, t5444: f64, t5448: f64, t5449: f64, t5455: f64, t5459: f64) -> (f64, f64, f64, f64) {
    let t7214 = t101 * t7213;
    let t7220 = t473 * t2363;
    let t7222 = t7220 * t483 * t485;
    let t7231 = -0.04789693604101844_f64 * t5440 - 0.001975389032890948_f64 * t7222 - 0.12602162889256446_f64 * t5442 - 0.06301081444628223_f64 * t5444 + t5448 + 0.12602162889256446_f64 * t5449 - t5455 + t5459 + t4258 - 0.02394846802050922_f64 * t4250 - 0.003950778065781896_f64 * t4153 - 0.0004954275694490498_f64 * t4156 - t4160 - t4163 - t4165 + 0.006584630109636494_f64 * t4168 + t4172;
    (t7214, t7220, t7222, t7231)
}
