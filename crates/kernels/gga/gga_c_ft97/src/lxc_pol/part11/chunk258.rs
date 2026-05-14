//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 258/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk258<F: Float>(t247: F, t263: F, t719: F, t767: F, t771: F, t773: F, t231: F, t294: F, t301: F, t342: F, t343: F, t10: F, t296: F, t351: F, t295: F, t668: F) -> (F, F, F, F, F, F) {
    let t776 = -t247 * t771 - t263 * t719 - 2.0 * t767 + 2.0 * t773;
    let t784 = t231 * t294;
    let t788 = t301 - t342 * t343 * t784 / 4.0;
    let t790 = t10 * t351 * t296;
    let t791 = t790 / 18.0;
    let t792 = t295 * t668;
    (t776, t784, t788, t790, t791, t792)
}
