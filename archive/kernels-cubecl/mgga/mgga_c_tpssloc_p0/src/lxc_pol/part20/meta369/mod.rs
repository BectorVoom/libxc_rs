//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1714;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1715;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta369<F: Float>(t52: F, t12606: F, t12874: F, t12877: F, t2244: F, t2250: F, t4087: F, t607: F, t76: F, t12873: F, t157: F, t182: F, t145: F, zeta_threshold: F, t185: F, t4195: F, t4194: F, t4303: F, t870: F, t262: F, t4119: F, t2553: F, t4315: F, t9717: F, t12850: F, t12854: F, t12860: F, t12861: F, t1877: F, t2522: F, t4310: F, t4314: F, t776: F, t868: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12886, t12887, t12889, t12890) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1714::<F>(t52, t12606, t12874, t12877, t2244, t2250, t4087, t607, t76, t12873, t157, t182, t145, zeta_threshold);
        let (t12891, t12892, t12894, t12895, t12899, t12903, t12906) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1715::<F>(t12890, t185, t2250, t4195, t4194, t4303, t870, t262, t4119, t2553, t4315, t9717);
        let t12907 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1716::<F>(t12850, t12854, t12860, t12861, t12889, t12891, t12894, t12895, t12899, t12903, t12906, t1877, t2522, t2553, t4310, t4314, t776, t868, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
    (t12886, t12887, t12889, t12890, t12891, t12892, t12894, t12895, t12899, t12903, t12906, t12907)
}
