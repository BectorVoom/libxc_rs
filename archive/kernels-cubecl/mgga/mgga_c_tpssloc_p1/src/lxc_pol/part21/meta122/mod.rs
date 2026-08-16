//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta122 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk824;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk825;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk826;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk827;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta122<F: Float>(t2770: F, t344: F, t2244: F, t2979: F, t337: F, t39: F, t1887: F, t60: F, t976: F, t984: F, t343: F, t883: F, t607: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2980, t2981, t2982, t2985, t2986) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk824::<F>(t2770, t344, t2244, t2979, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk825::<F>(t60, t976);
        let t2988 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk826::<F>(t2987, t984);
        let t2989 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk827::<F>(t343, t883);
        let t2990 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk828::<F>(t2989, t607);
    (t2980, t2981, t2982, t2985, t2986, t2987, t2988, t2989, t2990)
}
