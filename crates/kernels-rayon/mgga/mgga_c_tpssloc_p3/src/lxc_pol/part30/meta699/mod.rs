//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2247;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2248;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2249;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2250;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2251;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta699(t23097: f64, t232: f64, t67793: f64, t815: f64, t2628: f64, t5585: f64, t776: f64, t13228: f64, t4233: f64, t6605: f64, t25119: f64, t58557: f64, t22690: f64, t5527: f64, t81792: f64, t841: f64, t16805: f64, t1898: f64, t249: f64, t236: f64, t5584: f64, t23109: f64, t2632: f64, t81914: f64, t23110: f64, t5611: f64, t81877: f64, t81883: f64, t87308: f64, t87329: f64, t98744: f64, t98746: f64, t98750: f64, t98752: f64, t98754: f64, t5587: f64, t81886: f64, t23041: f64, t5619: f64, t16753: f64, t16928: f64, t25084: f64, t16851: f64, t221: f64, t87420: f64, t16944: f64, t25154: f64, t87407: f64, t81903: f64, t87331: f64, t87333: f64, t87336: f64, t87339: f64, t87342: f64, t87348: f64, t87364: f64, t87387: f64, t87402: f64, t92652: f64, t23127: f64, t5628: f64, t16985: f64, t6621: f64, t1516: f64, t87321: f64, t25068: f64, t4261: f64, t5624: f64, t23133: f64, t87340: f64, t16673: f64, t6620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98758, t98762, t98766, t98770) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2247(t23097, t232, t67793, t815, t2628, t5585, t776, t13228, t4233, t6605, t25119, t58557);
        let (t98774, t98777, t98779, t98782) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2248(t22690, t5527, t81792, t841, t16805, t1898, t249, t236, t5584, t23109, t2632, t81914);
        let t98795 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2249(t23109, t23110, t232, t236, t5611, t98779, t81877, t81883, t87308, t87329, t98744, t98746, t98750, t98752, t98754, t98758, t98762, t98766, t98770, t98774, t98777, t98782);
        let (t98796, t98798, t98801, t98803, t98808) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2250(t5587, t81886, t23041, t5619, t16753, t6605, t815, t16928, t25084, t16851, t221, t87420);
        let t98816 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2251(t16944, t221, t25154, t16851, t841, t87407, t81903, t87331, t87333, t87336, t87339, t87342, t87348, t87364, t87387, t87402, t92652, t98796, t98798, t98801, t98803, t98808);
        let (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2252(t23127, t5628, t16985, t6621, t1516, t87321, t25068, t4261, t5624, t23133, t87340, t16673, t6620);
    (t98795, t98816, t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832)
}
