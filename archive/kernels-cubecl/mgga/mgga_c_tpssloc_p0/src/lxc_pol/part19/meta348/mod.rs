//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1259;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1260;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1261;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1262;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1263;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1264;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta348<F: Float>(t10529: F, t690: F, t10571: F, t885: F, t9698: F, t2289: F, t2769: F, t39097: F, t10564: F, t123: F, t10216: F, t2244: F, t2250: F, t2768: F, t2770: F, t39103: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t41680 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1259::<F>(t10529, t690);
        let t41682 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1260::<F>(t10571, t690);
        let t41684 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1261::<F>(t885, t9698);
        let (t41687, t41688, t41690) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1262::<F>(t2289, t2769, t39097, t10564, t123);
        let (t41693, t41695) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1263::<F>(t10216, t2244, t2250, t10564, t123);
        let (t41697, t41699) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1264::<F>(t10216, t39097, t123, t2768);
        let (t41701, t41703) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1265::<F>(t2770, t39103, t123, t2768);
    (t41680, t41682, t41684, t41687, t41688, t41690, t41693, t41695, t41697, t41699, t41701, t41703)
}
