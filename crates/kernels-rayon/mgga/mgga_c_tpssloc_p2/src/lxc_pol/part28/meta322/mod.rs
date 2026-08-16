//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1252;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta322(t11820: f64, t1213: f64, t1226: f64, t3566: f64, t11552: f64, t221: f64, t456: f64, t1197: f64, t698: f64, t1174: f64, t135: f64, t3551: f64, t3556: f64, t3493: f64, t3612: f64, t11812: f64, t1243: f64, t10471: f64, t11715: f64, t11712: f64, t11721: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11821, t11825, t11834, t11836, t11838) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1252(t11820, t1213, t1226, t3566, t11552, t221, t456, t1197, t698, t1174, t135, t3551);
        let (t11839, t11842, t11871, t11877, t11881, t11883) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1253(t1174, t11838, t135, t3556, t3493, t3612, t11812, t1243, t10471, t11715, t11712, t11721, t6739);
    (t11821, t11825, t11834, t11836, t11839, t11842, t11871, t11877, t11881, t11883)
}
