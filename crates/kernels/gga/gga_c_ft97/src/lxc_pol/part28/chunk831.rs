//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 831/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk831<F: Float>(t136303: F, t5612: F, t22513: F, t5589: F, t14: F, t1669: F, t22755: F, t92354: F, t5522: F, t420: F, t5590: F, t92461: F, t32145: F, t92335: F, t173: F, t32151: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t136304 = t136303 * t5612;
    let t136305 = t22513 * t136304;
    let t136307 = sigma0 * t5589;
    let t136308 = t136307 * t14;
    let t136313 = t1669 * t22755 * t92354;
    let t136331 = t1669 * t5522 * t92354;
    let t136332 = t5590 * t420;
    let t136336 = t92461 * t420;
    let t136356 = t92335 * t32145;
    let t136359 = t136307 * t420;
    let t136363 = t32151 * t173;
    (t136304, t136305, t136308, t136313, t136331, t136332, t136336, t136356, t136359, t136363)
}
