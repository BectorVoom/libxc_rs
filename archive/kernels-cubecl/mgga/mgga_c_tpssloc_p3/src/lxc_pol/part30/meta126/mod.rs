//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk737;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk738;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk739;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk740;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta126<F: Float>(t390: F, t1878: F, t268: F, t405: F, t1091: F, t690: F, t1229: F, t154: F, t636: F, t2296: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3215, t3216) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk737::<F>(t390);
        let (t3236, t3237, t3238) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk738::<F>(t1878, t268, t405, t1091, t690);
        let t3240 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk739::<F>(t1229, t154);
        let (t3241, t3242) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk740::<F>(t636);
        let t3247 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk741::<F>(t2296);
    (t3215, t3216, t3236, t3237, t3238, t3240, t3241, t3242, t3247)
}
