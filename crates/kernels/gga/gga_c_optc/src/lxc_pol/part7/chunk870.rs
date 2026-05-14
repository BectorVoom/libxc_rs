//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 870/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk870<F: Float>(t8639: F, t8642: F, t8628: F, t8630: F, t8632: F, t8636: F, t8645: F, t8648: F, t8651: F, t8654: F, t8657: F, t8660: F, t8674: F, t8676: F, t8826: F, t1075: F) -> (F, F) {
    let t8831 = 0.16068111111111111111e1 * t8639;
    let t8832 = 0.46308888888888888888e0 * t8642;
    let t8841 = -0.34731666666666666667e0 * t8628 + 0.20839e0 * t8630 + 0.69463333333333333335e-1 * t8632 - 0.46308888888888888889e-1 * t8636 - t8831 - t8832 - 0.52945875e1 * t8645 + 0.94674375e0 * t8648 - 0.104195e0 * t8651 + 0.62517e0 * t8654 - 0.103295e1 * t8657 + 0.309885e1 * t8660 + 0.6311625e0 * t8674 + 0.3529725e1 * t8676;
    let t8842 = t8826 + t8841;
    let t8843 = t8842 * t1075;
    (t8842, t8843)
}
