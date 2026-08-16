//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta494 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1846;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1847;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1848;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1849;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta494<F: Float>(t28: F, t2161: F, t2250: F, t23820: F, t24916: F, t52: F, t607: F, t7402: F, t24562: F, t111: F, t7263: F, dens_threshold: F, rho1: F, zeta_threshold: F, t2113: F, t2319: F, t2363: F, t23844: F, t23846: F, t23848: F, t23850: F, t23852: F, t23854: F, t24543: F, t671: F, t7266: F, t113: F, t1266: F, t2165: F, t2167: F, t22460: F, t22467: F, t22482: F, t22563: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t24545: F, t24552: F, t3929: F, t510: F, t574: F, t650: F, t652: F, t672: F, t7264: F, t7271: F, t7408: F, t1393: F, t2114: F, t22577: F, t22580: F, t22583: F, t22587: F, t22594: F, t22599: F, t22605: F, t22608: F, t22610: F, t22612: F, t22614: F, t22616: F, t22618: F, t22950: F, t23833: F, t23835: F, t23837: F, t23860: F, t3652: F, t7412: F, t3: F, t112: F, t7415: F, t2169: F, t23886: F, t23888: F, t23890: F, t23892: F, t23895: F, t23898: F, t23900: F, t577: F, t7423: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24924, t24932) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1846::<F>(t28, t2161, t2250, t23820, t24916, t52, t607, t7402, t24562, t111, t7263, dens_threshold, rho1, zeta_threshold);
        let (t24935, t24939) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1847::<F>(t2113, t2319, t2363, t23844, t23846, t23848, t23850, t23852, t23854, t24543, t24932, t671, t7266);
        let t24949 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1848::<F>(t113, t1266, t2165, t2167, t22460, t22467, t22482, t22563, t2312, t2314, t2320, t2323, t2364, t24543, t24545, t24552, t24924, t24932, t24935, t24939, t3929, t510, t574, t650, t652, t672, t7264, t7266, t7271, t7408);
        let t24953 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1849::<F>(t1393, t2114, t22577, t22580, t22583, t22587, t22594, t22599, t22605, t22608, t22610, t22612, t22614, t22616, t22618, t22950, t23833, t23835, t23837, t23860, t3652, t7412);
        let (t24954, t24955, t24969, t24972, t24977) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1850::<F>(t24949, t24953, t3, t112, t7415, t111, t2169, t2319, t2363, t23886, t23888, t23890, t23892, t23895, t23898, t23900, t577, t671, t7423);
    (t24924, t24932, t24935, t24939, t24954, t24955, t24969, t24972, t24977)
}
