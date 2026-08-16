//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1731;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1732;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta521(t29274: f64, t29285: f64, t539: f64, t1807: f64, t7918: f64, t2085: f64, t6361: f64, t12021: f64, t2091: f64, t6439: f64, t1842: f64, t7936: f64, t3887: f64, t1375: f64, t1843: f64, t24071: f64, t26184: f64, t26198: f64, t26200: f64, t26345: f64, t27009: f64, t27068: f64, t28118: f64, t28193: f64, t28196: f64, t28201: f64, t5321: f64, t568: f64, t7925: f64, t553: f64, t24127: f64, t6388: f64, t1336: f64, t1814: f64, t2089: f64, t24099: f64, t26381: f64, t26393: f64, t26406: f64, t28132: f64, t28136: f64, t28140: f64, t28144: f64, t28150: f64, t544: f64, t6378: f64, t7934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29286, t29287, t29290, t29293, t29299, t29310) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1731(t29274, t29285, t539, t1807, t7918, t2085, t6361, t12021, t2091, t6439, t1842, t7936);
        let (t29311, t29314) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1732(t29310, t3887, t1375, t1843, t24071, t26184, t26198, t26200, t26345, t27009, t27068, t28118, t28193, t28196, t28201, t29287, t29290, t29293, t29299, t5321, t568, t7925);
        let (t29327, t29339, t29342) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1733(t29286, t553, t24127, t6388, t1336, t1814, t2089, t24099, t26381, t26393, t26406, t28132, t28136, t28140, t28144, t28150, t544, t6378, t7934);
    (t29286, t29287, t29290, t29293, t29299, t29311, t29314, t29327, t29339, t29342)
}
