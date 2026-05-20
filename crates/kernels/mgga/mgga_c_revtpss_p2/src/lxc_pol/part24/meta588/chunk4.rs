//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1840/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1840<F: Float>(t22466: F, t22852: F, t39483: F, t39520: F, t39528: F, t39531: F, t4139: F, t46963: F, t46970: F, t46972: F, t5532: F, t5536: F, t6816: F, t91956: F, t91958: F, t91959: F, t91960: F, t91961: F, t91962: F) -> F {
    let t92453 = -F::new(18.0) * t22466 * t4139 * t6816 + F::new(72.0) * t22852 * t5532 * t5536 - t39483 + t39520 - t39528 + t39531 - t46963 + t46970 - t46972 + t91956 + t91958 - t91959 - t91960 + t91961 - t91962;
    t92453
}
