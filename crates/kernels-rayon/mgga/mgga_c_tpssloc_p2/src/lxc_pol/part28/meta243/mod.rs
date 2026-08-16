//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1063;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1064;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1065;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1066;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta243(t6613: f64, t812: f64, t831: f64, t1899: f64, t838: f64, t234: f64, t59: f64, t240: f64, t849: f64, t1906: f64, t6547: f64, t214: f64, t225: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6614 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1063(t6613, t812);
        let (t6615, t6617, t6619, t6620) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1064(t6614, t831, t1899, t838, t234, t59, t240);
        let t6621 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1065(t6620, t812);
        let (t6622, t6635, t6637) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1066(t6621, t849, t1906, t6547, t214, t225);
        let t6638 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1067(t234, t252);
    (t6614, t6615, t6617, t6619, t6620, t6621, t6622, t6635, t6637, t6638)
}
