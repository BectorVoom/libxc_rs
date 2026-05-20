//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2150/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2150<F: Float>(t27384: F, t98763: F, t27375: F, t890: F, t27383: F, t1940: F, t1963: F, t2257: F, t2403: F, t25206: F, t25211: F, t25440: F, t25445: F, t27158: F, t27166: F, t27364: F, t27382: F, t27387: F, t7010: F, t7091: F, t7783: F, t7787: F, t92775: F, t92819: F, t98733: F, t98736: F, t98740: F, t98743: F, t98751: F, t98755: F, t98760: F) -> (F, F) {
    let t98764 = t98763 * t27384;
    let t98767 = t27375 * t890;
    let t98768 = t27383 * t98767;
    let t98776 = F::new(3.0) * t2403 * t7783 * t25211 + t1940 * t7783 * t2257 / F::new(2.0) - F::new(3.0) * t25206 * t98733 - t1940 * t7091 * t98736 / F::new(2.0) + t1940 * t25445 * t98740 - F::new(3.0) * t25206 * t98743 - t1940 * t25440 * t27387 + F::new(3.0) * t2403 * t27364 * t7010 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t98751 - t1940 * t7091 * t98755 / F::new(2.0) - F::new(3.0) * t27158 * t98760 + F::new(2.0) * t27382 * t98764 + F::new(6.0) * t25206 * t98768 - t1940 * t92775 * t7787 / F::new(2.0) - F::new(3.0) * t92819 * t27166;
    (t98767, t98776)
}
