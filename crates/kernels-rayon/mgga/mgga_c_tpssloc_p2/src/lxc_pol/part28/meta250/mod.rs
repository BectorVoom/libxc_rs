//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1087;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1088;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1089;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta250(t1336: f64, t6944: f64, t1354: f64, t1358: f64, t2003: f64, t552: f64, t59: f64, t240: f64, t1369: f64, t2010: f64, t6883: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6945 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1087(t1336, t6944);
        let (t6946, t6948, t6950, t6951) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1088(t1354, t6945, t1358, t2003, t552, t59, t240);
        let t6952 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1089(t1336, t6951);
        let (t6953, t6966, t6968) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1090(t1369, t6952, t2010, t6883, t552, t562);
    (t6945, t6946, t6948, t6950, t6951, t6952, t6953, t6966, t6968)
}
