//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 872/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk872<F: Float>(t2112: F, t34853: F, t1369: F, t28: F, t1009: F, t7318: F, t1008: F, t2035: F, t1013: F, t71: F, t420: F, t7195: F) -> (F, F, F, F, F, F, F, F) {
    let t34854 = t2112 * t34853;
    let t34856 = t1369 * t28 * t34854;
    let t34857 = t7318 * t1009;
    let t34864 = t2035 * t7318 * t1008;
    let t34868 = t2035 * t7318 * t1013;
    let t34871 = t71 * t1008;
    let t34872 = t420 * t34871;
    let t34873 = t7195 * t34872;
    (t34854, t34856, t34857, t34864, t34868, t34871, t34872, t34873)
}
