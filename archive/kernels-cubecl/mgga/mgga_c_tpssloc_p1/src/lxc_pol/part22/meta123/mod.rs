//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta123 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk832;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk833;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk834;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk835;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta123<F: Float>(t2375: F, t3684: F, t1294: F, t2371: F, t2528: F, t1284: F, t172: F, t763: F, t2535: F, t570: F, t515: F, t518: F, t215: F, t2559: F, t535: F, t1314: F, t782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3686, t3688, t3690, t3691) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk832::<F>(t2375, t3684, t1294, t2371, t2528, t1284, t172);
        let (t3692, t3695, t3700, t3701) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk833::<F>(t3691, t763, t1294, t2535, t570);
        let t3704 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk834::<F>(t515);
        let t3711 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk835::<F>(t518);
        let (t3725, t3726) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk836::<F>(t215, t2559, t535, t1314, t782);
    (t3686, t3688, t3690, t3691, t3692, t3695, t3700, t3701, t3704, t3711, t3725, t3726)
}
