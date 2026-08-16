//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk579;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk580;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk581;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk582;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk583;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk584;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta106<F: Float>(t3014: F, t340: F, t343: F, t974: F, t2955: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2982: F, t2986: F, t2991: F, t2996: F, t3000: F, t3011: F, t346: F, t973: F, t980: F, t987: F, t381: F, t1049: F, t990: F, t225: F, t991: F, t1008: F, t191: F, t349: F, t1011: F, t68: F, t371: F, t335: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3016, t3017, t3020) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk579::<F>(t3014, t340, t343, t974, t2955, t2958, t2960, t2969, t2972, t2975, t2982, t2986, t2991, t2996, t3000, t3011, t346, t973, t980, t987);
        let (t3021, t3023, t3026, t3030) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk580::<F>(t3020, t381, t1049, t990, t225, t991, t1008, t191);
        let (t3031, t3032) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk581::<F>(t3030, t349, t1011, t68);
        let t3033 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk582::<F>(t3031, t3032);
        let t3034 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk583::<F>(t371);
        let t3036 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk584::<F>(t3034, t335);
    (t3016, t3017, t3020, t3021, t3023, t3026, t3030, t3031, t3032, t3033, t3034, t3036)
}
