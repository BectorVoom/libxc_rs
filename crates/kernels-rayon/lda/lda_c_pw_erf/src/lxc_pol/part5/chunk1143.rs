//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1143/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1143(t3416: f64, t7680: f64, t1318: f64, t2334: f64, t4688: f64, t4758: f64, t21051: f64, t21056: f64, t21057: f64, t21058: f64, t21059: f64, t21060: f64, t21064: f64, t21066: f64, t21067: f64, t21069: f64, t21071: f64) -> (f64, f64, f64) {
    let t21073 = 16.0_f64 / 15.0_f64 * t3416 * t7680;
    let t21077 = 16.0_f64 / 15.0_f64 * t1318 * t4758 * t4688 * t2334;
    let t21078 = t21051 - t21056 + t21057 + t21058 + t21059 - t21060 + t21064 - t21066 + t21067 + t21069 - t21071 - t21073 + t21077;
    (t21073, t21077, t21078)
}
