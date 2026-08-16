//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta462 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1742;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1743;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1744;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1745;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta462<F: Float>(t225: F, t6625: F, t6576: F, t2752: F, t6665: F, t10143: F, t1914: F, t221: F, t2987: F, t1926: F, t344: F, t381: F, t1054: F, t883: F, t6733: F, t6686: F, t6712: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23278, t23281, t23290) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1742::<F>(t225, t6625, t6576, t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1743::<F>(t10143, t1914);
        let (t23326, t23327) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1744::<F>(t221, t2987, t1926);
        let (t23328, t23329) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1745::<F>(t344, t381, t225);
        let (t23330, t23336, t23346) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1746::<F>(t1054, t883, t381, t6733, t6686, t6712);
    (t23278, t23281, t23290, t23295, t23326, t23327, t23328, t23329, t23330, t23336, t23346)
}
