//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta253 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1096;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1097;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1098;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1099;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1100;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta253(t1860: f64, t7032: f64, t2031: f64, t6509: f64, t5: f64, t2032: f64, t6486: f64, t6492: f64, t6495: f64, t7026: f64, t112: f64, t111: f64, t2035: f64, t1266: f64, t2039: f64, t109: f64, t6528: f64, t6531: f64, t510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7034, t7035) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1096(t1860, t7032, t2031, t6509);
        let (t7039, t7040) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1097(t5, t1860, t2032, t6486, t6492, t6495, t7026, t7034, t7035, t112);
        let t7042 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1098(t111, t2035);
        let t7050 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1099(t1266, t2039);
        let (t7053, t7056) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1100(t109, t6528, t6531);
        let t7057 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1101(t510, t7056);
    (t7034, t7035, t7039, t7040, t7042, t7050, t7053, t7056, t7057)
}
