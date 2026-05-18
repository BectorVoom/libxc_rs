//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1273/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1273<F: Float>(t11035: F, t11038: F, t12503: F, t12504: F, t12505: F, t12520: F, t12523: F, t12524: F, t12525: F, t12528: F, t12530: F, t12533: F, t12535: F) -> F {
    let t15019 = t12503 + t12504 + t12505 - t12520 + t12523 + F::new(0.09973633333333333) * t11035 + t12524 - t12525 - t11038 - t12528 - t12530 - t12533 - t12535;
    t15019
}
