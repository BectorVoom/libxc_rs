//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1328/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1328<F: Float>(t19: F, t5944: F, t729: F, t734: F, t11468: F, t11470: F, t11472: F, t11475: F, t11476: F, t11516: F, t11530: F, t11556: F, t11599: F, t14423: F, t14425: F, t14426: F, t14475: F, t14517: F, t14908: F, t15281: F, t312: F, t8414: F, t8417: F, t8423: F, t8427: F, t8432: F, t8437: F, t8445: F, t8449: F, t8469: F) -> F {
    let t15288 = t5944 * t729 * t19 * t734;
    let t15290 = t8414 + t8417 + t11468 - t11470 + t11472 + t11475 + t8423 - t8427 + t8432 + t8437 - t11476 + t8445 - t8449 - (t11516 + t11530 + t11556 + t11599 + t14475 + t14517 + t14908 + t15281) * t312 + t14423 - F::cast_from(1.232289865202_f64) * t15288 + t14425 - t14426 + t8469;
    t15290
}
