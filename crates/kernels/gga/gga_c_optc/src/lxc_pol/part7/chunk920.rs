//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 920/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk920<F: Float>(t8639: F, t8642: F, t8628: F, t8630: F, t8632: F, t8636: F, t8645: F, t8648: F, t8651: F, t8654: F, t8657: F, t8660: F, t8674: F, t8676: F) -> F {
    let t8727 = F::new(0.93932222222222222223e0) * t8639;
    let t8728 = F::new(0.36793333333333333333e0) * t8642;
    let t8737 = -F::new(0.27595e0) * t8628 + F::new(0.16557e0) * t8630 + F::new(0.5519e-1) * t8632 - F::new(0.36793333333333333333e-1) * t8636 - t8727 - t8728 - F::new(0.3883875e1) * t8645 + F::new(0.247573125e0) * t8648 - F::new(0.82785e-1) * t8651 + F::new(0.49671e0) * t8654 - F::new(0.60384999999999999999e0) * t8657 + F::new(0.181155e1) * t8660 + F::new(0.16504875e0) * t8674 + F::new(0.258925e1) * t8676;
    t8737
}
