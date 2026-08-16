//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1322/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1322(t21393: f64, t21396: f64, t21401: f64, t21403: f64, t21406: f64, t21409: f64, t21413: f64, t21417: f64, t21423: f64, t21426: f64, t21427: f64, t21428: f64, t21430: f64) -> f64 {
    let t23246 = t21393 - t21396 + t21401 - t21403 + t21406 - t21409 + t21413 + t21417 + t21423 + t21426 - t21427 + t21428 - t21430;
    t23246
}
