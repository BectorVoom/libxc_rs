//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 712/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk712(t352: f64, t6365: f64, t2017: f64, t571: f64, t2411: f64, t549: f64, t1318: f64, t593: f64, t3832: f64, t2334: f64, t3604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6366 = t6365 * t352;
    let t6367 = t2017 * t6366;
    let t6369 = 4.0_f64 / 27.0_f64 * t571 * t6367;
    let t6370 = t2411 * t549;
    let t6371 = t2017 * t6370;
    let t6373 = 8.0_f64 / 27.0_f64 * t1318 * t6371;
    let t6374 = t2411 * t593;
    let t6375 = t3832 * t6374;
    let t6377 = 4.0_f64 / 27.0_f64 * t571 * t6375;
    let t6378 = t3604 * t2334;
    let t6379 = t6378 * t352;
    (t6366, t6367, t6369, t6370, t6371, t6373, t6374, t6375, t6377, t6378, t6379)
}
