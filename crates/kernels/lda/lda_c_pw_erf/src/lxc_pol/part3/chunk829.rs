//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 829/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk829<F: Float>(t5795: F, t647: F, t1432: F, t850: F, t256: F, t1427: F, t2260: F, t4464: F, t4466: F, t4468: F, t4470: F, t4471: F, t4472: F, t4473: F, t4474: F, t4478: F, t4482: F, t4486: F, t5788: F, t5793: F) -> (F, F) {
    let t5797 = F::cast_from(0.12155555555555556_f64) * t5795 * t647;
    let t5798 = t850 * t1432;
    let t5799 = t5798 * t256;
    let t5801 = t2260 * t1427;
    let t5803 = t5788 * t256 / F::cast_from(3.0_f64) + t5793 + t5797 + t5799 / F::cast_from(3.0_f64) + F::cast_from(0.12155555555555556_f64) * t5801 - t4464 - t4466 + t4468 + t4470 + t4471 + t4472 + t4473 + t4474 - t4478 - t4482 - t4486;
    (t5798, t5803)
}
