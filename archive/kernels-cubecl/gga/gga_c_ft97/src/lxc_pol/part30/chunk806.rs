//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 806/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk806<F: Float>(t2862: F, t7584: F, t882: F, t296: F, t34019: F, t34017: F, t34015: F, t34054: F, t7611: F, t840: F, t319: F, t33953: F) -> (F, F, F, F, F, F, F) {
    let t34130 = t2862 * t882 * t7584;
    let t34133 = t296 * t34019;
    let t34136 = t296 * t34017;
    let t34139 = t296 * t34015;
    let t34142 = t296 * t34054;
    let t34146 = t840 * t882 * t7611;
    let t34150 = t840 * t319 * t33953;
    (t34130, t34133, t34136, t34139, t34142, t34146, t34150)
}
