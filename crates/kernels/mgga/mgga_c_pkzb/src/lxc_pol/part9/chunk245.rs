//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 245/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk245<F: Float>(t179: F, t655: F, t780: F, t276: F, t279: F, t299: F, t303: F, t735: F, t741: F, t744: F, t757: F, t763: F, t771: F, t777: F) -> (F, F) {
    let t782 = t179 * t780 * t655;
    let t785 = -t735 * t279 / F::new(36.0) + t741 - t276 * t744 / F::new(96.0) + F::new(0.21437009059034868486e-3) * t757 * t763 - F::new(0.11433071498151929859e-2) * t771 * t303 + t777 - F::new(0.42874018118069736972e-3) * t299 * t782;
    (t782, t785)
}
