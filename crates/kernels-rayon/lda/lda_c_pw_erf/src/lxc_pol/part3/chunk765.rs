//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 765/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk765(t2058: f64, t331: f64, t2055: f64, t1371: f64, t4680: f64, t3587: f64, t4666: f64, t4676: f64, t4693: f64, t589: f64, t4689: f64, t4659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4998 = 0.017777777777777778_f64 * t331 * t2058;
    let t5000 = 0.002962962962962963_f64 * t331 * t2055;
    let t5001 = t1371 * t4680;
    let t5004 = t3587 * t4666;
    let t5007 = t1371 * t4676;
    let t5010 = t589 * t4693;
    let t5013 = t589 * t4689;
    let t5017 = 0.015996296296296297_f64 * t4659;
    (t4998, t5000, t5001, t5004, t5007, t5010, t5013, t5017)
}
