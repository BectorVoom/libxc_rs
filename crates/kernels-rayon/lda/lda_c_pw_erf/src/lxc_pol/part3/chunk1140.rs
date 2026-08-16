//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1140/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1140(t13351: f64, t3669: f64, t571: f64, t816: f64, t9286: f64, t10654: f64, t1318: f64, t2034: f64, t2011: f64, t3742: f64, t13325: f64, t13327: f64, t13329: f64, t13334: f64, t13338: f64, t13340: f64, t13342: f64, t13347: f64, t13349: f64) -> (f64, f64, f64, f64, f64) {
    let t13352 = 16.0_f64 / 45.0_f64 * t13351;
    let t13356 = 8.0_f64 / 15.0_f64 * t571 * t9286 * t816 * t3669;
    let t13358 = t1318 * t10654 * t2034;
    let t13359 = 16.0_f64 / 135.0_f64 * t13358;
    let t13361 = 8.0_f64 / 15.0_f64 * t3742 * t2011;
    let t13362 = -t13325 + t13327 - t13329 + t13334 + t13338 - t13340 - t13342 + t13347 + t13349 + t13352 - t13356 - t13359 - t13361;
    (t13352, t13356, t13359, t13361, t13362)
}
