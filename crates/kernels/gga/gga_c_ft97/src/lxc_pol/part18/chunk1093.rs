//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1093/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1093<F: Float>(t1681: F, t5539: F, t37481: F, t5551: F, t5555: F, t1611: F, t58: F, t1696: F, t22737: F, t22742: F, t397: F, t22632: F, t22761: F, t22762: F, t5598: F, t5599: F, t92433: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t92699 = t5539 * t1681;
    let t92710 = t37481 * t5551 * t5555;
    let t92715 = t1611 * sigma0 * t58;
    let t92782 = t22737 * t1696;
    let t92786 = t22742 * t397;
    let t92791 = t22761 * t22632 * t22762;
    let t92794 = t5598 * t92433 * t5599;
    (t92699, t92710, t92715, t92782, t92786, t92791, t92794)
}
