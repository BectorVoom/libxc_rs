//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1297/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1297<F: Float>(t18427: F, t18430: F, t18433: F, t18445: F, t18448: F, t18765: F, t18766: F, t22190: F, t22193: F, t22196: F, t22199: F, t22202: F, t22205: F, t22207: F, t22209: F, t22215: F, t22217: F, t22220: F, t22222: F) -> F {
    let t22795 = F::new(0.794188125e1) * t22190 - F::new(0.473371875e0) * t22193 - F::new(0.6618234375e1) * t22196 + F::new(0.2366859375e0) * t22199 - F::new(0.52945875e1) * t22202 + F::new(0.94674375e0) * t22205 - F::new(0.52945875e1) * t22207 - F::new(0.17648625e1) * t22209 + t18765 - F::new(0.48204333333333333334e1) * t18427 + F::new(0.20659e1) * t18430 - F::new(0.516475e0) * t18433 + t18766 + F::new(0.104195e1) * t18448 + F::new(0.94674375e0) * t22215 + F::new(0.31558125e0) * t22217 + F::new(0.794188125e1) * t22220 - F::new(0.473371875e0) * t22222 - F::new(0.27785333333333333333e1) * t18445;
    t22795
}
