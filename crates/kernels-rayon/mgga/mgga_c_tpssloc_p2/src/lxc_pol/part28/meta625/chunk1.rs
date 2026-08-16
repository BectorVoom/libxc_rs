//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1951/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1951(t1324: f64, t254: f64, t22724: f64, t26344: f64, t22643: f64, t7691: f64, t81195: f64, t1388: f64, t25988: f64, t1845: f64, t3719: f64, t22573: f64, t7684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91505 = t1324 * t254;
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91565 = t25988 * t1388;
    let t91603 = t1845 * t3719;
    let t91655 = t7684 * t22573;
    (t91505, t91531, t91548, t91565, t91603, t91655)
}
