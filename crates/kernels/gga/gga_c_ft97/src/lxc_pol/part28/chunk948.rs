//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 948/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk948<F: Float>(t136825: F, t32138: F, t32141: F, t1608: F, t1710: F, t25: F, t5555: F, t32280: F, t32279: F, t22696: F, t32258: F, t11: F, t171: F, t397: F) -> (F, F, F, F, F, F) {
    let t136827 = t32138 * t136825 * t32141;
    let t136831 = t1608 * t1710 * t25 * t5555;
    let t136840 = t136825 * t32280;
    let t136841 = t32279 * t136840;
    let t136843 = t22696 * t32258;
    let t136847 = t11 * t397 * t171;
    (t136827, t136831, t136840, t136841, t136843, t136847)
}
