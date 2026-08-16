//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1015/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1015(t11180: f64, t2320: f64, t1174: f64, t3743: f64, t6149: f64, t3041: f64, t3747: f64, t11155: f64, t6156: f64, t7955: f64, t9782: f64, t834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11181 = t11180 * t2320;
    let t11184 = t3743 * t1174;
    let t11185 = t6149 * t11184;
    let t11187 = t3041 * t3747;
    let t11190 = -t6156 + 4.0_f64 / 3.0_f64 * t7955 - t9782 + t11155;
    let t11191 = t834 * t11190;
    (t11181, t11184, t11185, t11187, t11190, t11191)
}
