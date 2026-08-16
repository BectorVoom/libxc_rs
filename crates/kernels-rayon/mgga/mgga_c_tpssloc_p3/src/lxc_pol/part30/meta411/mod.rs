//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta411 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1558;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1559;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1560;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1561;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1562;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1563;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta411(t11153: f64, t5392: f64, t607: f64, t3240: f64, t123: f64, t3966: f64, t4723: f64, t5976: f64, t690: f64, t5971: f64, t1088: f64, t4728: f64, t5980: f64, t3242: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18211, t18213) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1558(t11153, t5392, t607, t3240, t123);
        let (t18215, t18217) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1559(t3966, t4723, t3240, t123);
        let t18219 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1560(t5976, t690);
        let (t18221, t18223) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1561(t5971, t607, t1088, t123);
        let (t18225, t18227) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1562(t3966, t4728, t1088, t123);
        let t18229 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1563(t5980, t690);
        let (t18232, t18234) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1564(t3242, t5398, t607, t3240, t123);
    (t18211, t18213, t18215, t18217, t18219, t18221, t18223, t18225, t18227, t18229, t18232, t18234)
}
