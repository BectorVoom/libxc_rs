//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1173/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1173(t21397: f64, t494: f64, t3965: f64, t4494: f64, t12314: f64, t6756: f64, t6492: f64, t6762: f64, t6352: f64, t6766: f64, t348: f64, t12439: f64, t4488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21398 = t21397 * t494;
    let t21401 = 16.0_f64 / 15.0_f64 * t3965 * t4494 * t21398;
    let t21403 = 16.0_f64 / 15.0_f64 * t12314 * t6756;
    let t21406 = 16.0_f64 / 5.0_f64 * t3965 * t6762 * t6492;
    let t21409 = 16.0_f64 / 3.0_f64 * t3965 * t6766 * t6352;
    let t21410 = t21397 * t348;
    let t21413 = 8.0_f64 / 3.0_f64 * t4488 * t12439 * t21410;
    (t21398, t21401, t21403, t21406, t21409, t21410, t21413)
}
