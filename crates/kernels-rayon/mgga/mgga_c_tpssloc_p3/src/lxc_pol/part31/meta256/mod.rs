//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1070;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1071;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1072;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta256(t1351: f64, t562: f64, t550: f64, t6976: f64, t1992: f64, t1372: f64, t1998: f64, t214: f64, t1985: f64, t1388: f64, t3701: f64, t33: f64, t63: f64, t2240: f64, t625: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6978, t6979, t6980, t6982, t6983, t6984, t6999, t7025) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1070(t1351, t562, t550, t6976, t1992, t1372, t1998, t214, t1985, t1388, t3701, t33, t63);
        let t7026 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1071(t2240, t7025);
        let t7031 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1072(t625, t67);
        let t7032 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1073(t1864, t7031);
    (t6978, t6979, t6980, t6982, t6983, t6984, t6999, t7025, t7026, t7031, t7032)
}
