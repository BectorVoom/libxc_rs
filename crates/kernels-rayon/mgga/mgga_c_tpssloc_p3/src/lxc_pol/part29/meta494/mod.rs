//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1846;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1847;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1848;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1849;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta494(t28: f64, t2161: f64, t2250: f64, t23820: f64, t24916: f64, t52: f64, t607: f64, t7402: f64, t24562: f64, t111: f64, t7263: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t2113: f64, t2319: f64, t2363: f64, t23844: f64, t23846: f64, t23848: f64, t23850: f64, t23852: f64, t23854: f64, t24543: f64, t671: f64, t7266: f64, t113: f64, t1266: f64, t2165: f64, t2167: f64, t22460: f64, t22467: f64, t22482: f64, t22563: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t24545: f64, t24552: f64, t3929: f64, t510: f64, t574: f64, t650: f64, t652: f64, t672: f64, t7264: f64, t7271: f64, t7408: f64, t1393: f64, t2114: f64, t22577: f64, t22580: f64, t22583: f64, t22587: f64, t22594: f64, t22599: f64, t22605: f64, t22608: f64, t22610: f64, t22612: f64, t22614: f64, t22616: f64, t22618: f64, t22950: f64, t23833: f64, t23835: f64, t23837: f64, t23860: f64, t3652: f64, t7412: f64, t3: f64, t112: f64, t7415: f64, t2169: f64, t23886: f64, t23888: f64, t23890: f64, t23892: f64, t23895: f64, t23898: f64, t23900: f64, t577: f64, t7423: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24924, t24932) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1846(t28, t2161, t2250, t23820, t24916, t52, t607, t7402, t24562, t111, t7263, dens_threshold, rho1, zeta_threshold);
        let (t24935, t24939) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1847(t2113, t2319, t2363, t23844, t23846, t23848, t23850, t23852, t23854, t24543, t24932, t671, t7266);
        let t24949 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1848(t113, t1266, t2165, t2167, t22460, t22467, t22482, t22563, t2312, t2314, t2320, t2323, t2364, t24543, t24545, t24552, t24924, t24932, t24935, t24939, t3929, t510, t574, t650, t652, t672, t7264, t7266, t7271, t7408);
        let t24953 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1849(t1393, t2114, t22577, t22580, t22583, t22587, t22594, t22599, t22605, t22608, t22610, t22612, t22614, t22616, t22618, t22950, t23833, t23835, t23837, t23860, t3652, t7412);
        let (t24954, t24955, t24969, t24972, t24977) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1850(t24949, t24953, t3, t112, t7415, t111, t2169, t2319, t2363, t23886, t23888, t23890, t23892, t23895, t23898, t23900, t577, t671, t7423);
    (t24924, t24932, t24935, t24939, t24954, t24955, t24969, t24972, t24977)
}
