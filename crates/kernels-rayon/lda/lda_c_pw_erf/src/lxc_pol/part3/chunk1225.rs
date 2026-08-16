//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1225/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1225(t339: f64, t5685: f64, t11460: f64, t85: f64, t8464: f64, t11466: f64, t11468: f64, t11470: f64, t11472: f64, t11475: f64, t11476: f64, t8414: f64, t8417: f64, t8423: f64, t8427: f64, t8432: f64, t8437: f64, t8445: f64, t8449: f64) -> (f64, f64, f64, f64) {
    let t14423 = 24.0_f64 * t339 * t5685;
    let t14425 = 0.019751789702565206_f64 * t11460 * t85;
    let t14426 = 0.0017090784700969615_f64 * t8464;
    let t14427 = t11466 + t8414 + t8417 + t11468 - t11470 + t11472 + t11475 + t8423 - t8427 + t8432 + t8437 - t11476 + t8445 - t8449 + t14423 + t14425 - t14426;
    (t14423, t14425, t14426, t14427)
}
