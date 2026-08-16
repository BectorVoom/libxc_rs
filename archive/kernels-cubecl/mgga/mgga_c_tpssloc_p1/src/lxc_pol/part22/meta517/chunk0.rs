//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1982/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1982<F: Float>(t21812: F, t21815: F, t21829: F, t21832: F, t21835: F, t21956: F, t21958: F, t21960: F, t21963: F, t22224: F, t22226: F, t11292: F, t21906: F) -> (F, F) {
    let t22227 = t21956 + t21958 + t21960 - t21963 + t21812 + t21815 + t21829 - t21832 + t21835 - t22224 - t22226;
    let t22228 = t11292 * t21906;
    (t22227, t22228)
}
