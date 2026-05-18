//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1070/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1070<F: Float>(t12536: F, t525: F, t11636: F, t12508: F, t12509: F, t12514: F, t12520: F, t12523: F, t12524: F, t12525: F, t12528: F, t12530: F, t12533: F, t12535: F, t225: F, t231: F) -> (F, F) {
    let t12538 = F::new(4.0) / F::new(15.0) * t12536 * t525;
    let t12539 = t12508 + F::new(4.0) / F::new(3.0) * t12509 + F::new(4.0) / F::new(3.0) * t11636 * t225 * t231 + F::new(4.0) * t12514 - t12520 + t12523 + t12524 - t12525 - t12528 - t12530 - t12533 - t12535 + t12538;
    (t12538, t12539)
}
