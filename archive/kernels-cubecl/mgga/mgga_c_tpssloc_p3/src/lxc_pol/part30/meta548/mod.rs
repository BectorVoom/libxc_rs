//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1900;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1901;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta548<F: Float>(t109: F, t1873: F, t28002: F, t4028: F, t7467: F, t5493: F, t88: F, t7676: F, t22473: F, t5464: F, t5488: F, t6530: F, t22469: F, t27166: F, t1268: F, t1458: F, t24999: F, t27993: F, t27996: F, t28001: F, t6517: F, t510: F, t652: F, t7685: F, t7756: F, t89: F, t1874: F, t7458: F, t7461: F, t7468: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28004, t28006, t28007, t28009, t28011, t28017) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1900::<F>(t109, t1873, t28002, t4028, t7467, t5493, t88, t7676, t22473, t5464, t5488, t6530, t22469, t27166);
        let t28020 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1901::<F>(t1268, t28017, t1458, t24999, t27993, t27996, t28001, t28004, t28006, t28009, t28011, t5493, t6517);
        let (t28025, t28027, t28029, t28030, t28032, t28034, t28036) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1902::<F>(t28017, t510, t652, t7685, t7756, t5493, t89, t1874, t7458, t7461, t4028, t7468);
    (t28007, t28017, t28020, t28025, t28027, t28029, t28030, t28032, t28034, t28036)
}
