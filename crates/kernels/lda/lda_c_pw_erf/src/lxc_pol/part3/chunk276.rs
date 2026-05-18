//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 276/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk276<F: Float>(t231: F, t517: F, t570: F, t606: F, t613: F, t615: F, t788: F, t797: F, t801: F, t810: F, t815: F, t824: F, t828: F, t837: F, t838: F) -> F {
    let t841 = t788 + t797 + t517 + t801 - t810 + t815 + t824 + t570 + t828 - t837 + F::new(4.0) / F::new(3.0) * t838 * t231 + t606 + t613 + t615;
    t841
}
