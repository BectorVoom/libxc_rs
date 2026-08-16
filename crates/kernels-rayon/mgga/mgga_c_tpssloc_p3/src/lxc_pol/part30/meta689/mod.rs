//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2195;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2196;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2197;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta689(t24996: f64, t97890: f64, t28860: f64, t6876: f64, t1307: f64, t6324: f64, t22574: f64, t26162: f64, t28835: f64, t28830: f64, t24995: f64, t8643: f64, t1442: f64, t1869: f64, t19289: f64, t25958: f64, t33085: f64, t4073: f64, t6287: f64, t6515: f64, t672: f64, t96686: f64, t97862: f64, t97865: f64, t97869: f64, t97871: f64, t97874: f64, t97878: f64, t97880: f64, t97887: f64, t97889: f64, t74060: f64, t1388: f64, t1983: f64, t28238: f64, t6999: f64, t75214: f64, t12461: f64, t7752: f64, t26161: f64, t26163: f64, t24991: f64, t7685: f64, t25988: f64, t33136: f64, t28823: f64, t1874: f64, t96709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97892, t97893, t97897, t97899, t97905) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2195(t24996, t97890, t28860, t6876, t1307, t6324, t22574, t26162, t28835, t28830, t24995, t8643);
        let t97906 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2196(t1442, t1869, t19289, t25958, t33085, t4073, t6287, t6515, t672, t96686, t97862, t97865, t97869, t97871, t97874, t97878, t97880, t97887, t97889, t97892, t97893, t97897, t97899, t97905);
        let (t97910, t97914, t97916, t97919, t97920) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2197(t22574, t74060, t8643, t1388, t28830, t26162, t1983, t28238, t6999, t75214, t12461, t7752);
        let (t97923, t97925, t97928, t97930, t97932) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2198(t26161, t26163, t97920, t24991, t7685, t22574, t25988, t33136, t28823, t6876, t1874, t96709);
    (t97906, t97910, t97914, t97916, t97919, t97923, t97925, t97928, t97930, t97932)
}
