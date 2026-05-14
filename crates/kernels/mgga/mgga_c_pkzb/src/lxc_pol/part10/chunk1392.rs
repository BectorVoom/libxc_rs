//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1392/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1392<F: Float>(t27512: F, t27516: F, t27519: F, t27521: F, t27523: F, t27525: F, t27527: F, t27530: F, t27948: F, t27950: F, t27952: F, t27954: F, t27956: F, t3147: F, t8303: F, t10168: F, t2321: F, t898: F) -> (F, F, F) {
    let t27957 = -t27512 - t27516 - t27519 + t27521 + t27523 - t27525 + t27527 + t27530 + t27948 - t27950 - t27952 - t27954 + t27956;
    let t27960 = 0.70178683471615754484e1 * t3147 * t8303;
    let t27963 = 0.6233709278045326953e3 * t898 * t10168 * t2321;
    (t27957, t27960, t27963)
}
