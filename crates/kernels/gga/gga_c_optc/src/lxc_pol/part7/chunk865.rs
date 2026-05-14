//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 865/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk865<F: Float>(t8639: F, t8642: F, t8628: F, t8630: F, t8632: F, t8636: F, t8645: F, t8648: F, t8651: F, t8654: F, t8657: F, t8660: F, t8674: F, t8676: F, t8722: F) -> (F,) {
    let t8727 = 0.93932222222222222223e0 * t8639;
    let t8728 = 0.36793333333333333333e0 * t8642;
    let t8737 = -0.27595e0 * t8628 + 0.16557e0 * t8630 + 0.5519e-1 * t8632 - 0.36793333333333333333e-1 * t8636 - t8727 - t8728 - 0.3883875e1 * t8645 + 0.247573125e0 * t8648 - 0.82785e-1 * t8651 + 0.49671e0 * t8654 - 0.60384999999999999999e0 * t8657 + 0.181155e1 * t8660 + 0.16504875e0 * t8674 + 0.258925e1 * t8676;
    let t8738 = t8722 + t8737;
    (t8738,)
}
