//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1105/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1105(t10031: f64, t3977: f64, t5155: f64, t3974: f64, t4515: f64, t12900: f64, t12902: f64, t12903: f64, t12907: f64, t12909: f64, t12913: f64, t12915: f64, t12919: f64, t12923: f64, t12925: f64, t12927: f64) -> (f64, f64, f64, f64) {
    let t12928 = 32.0_f64 / 45.0_f64 * t10031;
    let t12929 = t5155 * t3977;
    let t12932 = 32.0_f64 / 15.0_f64 * t3974 * t4515 * t12929;
    let t12933 = -t12900 + t12902 - t12903 + t12907 + t12909 - t12913 - t12915 - t12919 + t12923 + t12925 + t12927 - t12928 + t12932;
    (t12928, t12929, t12932, t12933)
}
