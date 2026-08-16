//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1406;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1407;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1408;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1409;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1410;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1411;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta391(t3966: f64, t4723: f64, t3240: f64, t123: f64, t5976: f64, t690: f64, t5971: f64, t607: f64, t1088: f64, t4728: f64, t5980: f64, t3242: f64, t5398: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18215, t18217) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1406(t3966, t4723, t3240, t123);
        let t18219 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1407(t5976, t690);
        let (t18221, t18223) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1408(t5971, t607, t1088, t123);
        let (t18225, t18227) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1409(t3966, t4728, t1088, t123);
        let t18229 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1410(t5980, t690);
        let (t18232, t18234) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1411(t3242, t5398, t607, t3240, t123);
        let (t18237, t18239) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1412(t3247, t5398, t607, t1088, t123);
    (t18215, t18217, t18219, t18221, t18223, t18225, t18227, t18229, t18232, t18234, t18237, t18239)
}
