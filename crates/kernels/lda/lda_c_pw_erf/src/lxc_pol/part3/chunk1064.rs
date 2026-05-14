//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1064/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1064<F: Float>(t339: F, t5685: F, t11460: F, t85: F, t8464: F, t11466: F, t11468: F, t11470: F, t11472: F, t11475: F, t11476: F, t8414: F, t8417: F, t8423: F, t8427: F, t8432: F, t8437: F, t8445: F, t8449: F) -> (F, F, F, F) {
    let t14423 = 24.0 * t339 * t5685;
    let t14425 = 0.019751789702565206 * t11460 * t85;
    let t14426 = 0.0017090784700969615 * t8464;
    let t14427 = t11466 + t8414 + t8417 + t11468 - t11470 + t11472 + t11475 + t8423 - t8427 + t8432 + t8437 - t11476 + t8445 - t8449 + t14423 + t14425 - t14426;
    (t14423, t14425, t14426, t14427)
}
