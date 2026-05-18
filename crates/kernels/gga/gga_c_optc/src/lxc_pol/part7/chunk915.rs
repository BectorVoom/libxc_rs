//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 915/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk915<F: Float>(t1038: F, t8673: F, t8628: F, t8630: F, t8632: F, t8636: F, t8640: F, t8643: F, t8645: F, t8648: F, t8651: F, t8654: F, t8657: F, t8660: F, t8674: F) -> (F, F) {
    let t8676 = t1038 * t8673;
    let t8678 = -F::new(0.27385555555555555556e0) * t8628 + F::new(0.16431333333333333333e0) * t8630 + F::new(0.5477111111111111111e-1) * t8632 - F::new(0.36514074074074074075e-1) * t8636 - t8640 - t8643 - F::new(0.28483875e1) * t8645 + F::new(0.46074375e0) * t8648 - F::new(0.82156666666666666668e-1) * t8651 + F::new(0.49293999999999999999e0) * t8654 - F::new(0.59793333333333333333e0) * t8657 + F::new(0.17938e1) * t8660 + F::new(0.3071625e0) * t8674 + F::new(0.1898925e1) * t8676;
    (t8676, t8678)
}
