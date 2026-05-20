//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1903/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1903<F: Float>(t25978: F, t5614: F, t5622: F, t94443: F, t13769: F, t240: F, t2661: F, t7269: F, t13756: F, t7271: F, t13760: F, t25972: F) -> (F, F, F, F, F) {
    let t98146 = t25978 * t5614;
    let t98148 = t94443 * t5622;
    let t98152 = t2661 * t7269 * t240 * t13769;
    let t98154 = t7271 * t13756;
    let t98156 = t25972 * t13760;
    (t98146, t98148, t98152, t98154, t98156)
}
