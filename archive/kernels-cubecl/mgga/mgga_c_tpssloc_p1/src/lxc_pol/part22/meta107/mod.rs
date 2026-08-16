//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk725;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk726;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk727;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk728;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk729;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk730;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk731;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta107<F: Float>(t2978: F, t974: F, t2770: F, t344: F, t337: F, t39: F, t1887: F, t60: F, t976: F, t984: F, t343: F, t883: F, t607: F, t2775: F, t2822: F, t225: F, t991: F, t1008: F, t191: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2979 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk725::<F>(t2978, t974);
        let (t2980, t2986) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk726::<F>(t2770, t344, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk727::<F>(t60, t976);
        let t2988 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk728::<F>(t2987, t984);
        let t2989 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk729::<F>(t343, t883);
        let t2990 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk730::<F>(t2989, t607);
        let (t2994, t3003, t3026) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk731::<F>(t2775, t344, t2822, t225, t991);
        let t3030 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk732::<F>(t1008, t191);
    (t2979, t2980, t2986, t2987, t2988, t2989, t2990, t2994, t3003, t3026, t3030)
}
