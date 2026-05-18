//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1302/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1302<F: Float>(t1306: F, t31186: F, t31188: F, t31190: F, t31196: F, t31198: F, t31599: F, t31604: F, t31608: F, t31610: F, t31612: F, t3282: F, t9759: F) -> F {
    let t31616 = -F::new(3.0) * t1306 * t3282 * t9759 + t31186 + t31188 + t31190 - t31196 + t31198 + t31599 - t31604 - t31608 + t31610 - t31612;
    t31616
}
