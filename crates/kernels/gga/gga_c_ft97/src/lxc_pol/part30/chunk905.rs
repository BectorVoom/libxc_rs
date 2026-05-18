//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 905/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk905<F: Float>(t255: F, t675: F, t2371: F, t2492: F, t3977: F, t10052: F, t737: F, t2372: F, t754: F, t10: F, t16: F, t2404: F) -> (F, F, F, F, F, F) {
    let t53798 = t675 * t255;
    let t53891 = t2371 * t255;
    let t53923 = t2492 * t3977;
    let t53927 = t737 * t10052;
    let t53942 = t2372 * t754;
    let t54032 = t10 * t16 * t2404;
    (t53798, t53891, t53923, t53927, t53942, t54032)
}
