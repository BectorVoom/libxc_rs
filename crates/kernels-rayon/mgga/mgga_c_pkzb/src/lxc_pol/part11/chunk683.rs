//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 683/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk683(t42: f64, t987: f64, t13: f64, t25: f64, t1448: f64, t30: f64, t14: f64, t8: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4218 = t987 * t42;
    let t4494 = t13 * t13;
    let t4635 = t25 * t25;
    let t4772 = t30 * t1448;
    let t4793 = t14 * t13;
    let t4794 = 1.0_f64 / t4793;
    let t4803 = t8 * t82;
    (t4218, t4494, t4635, t4772, t4794, t4803)
}
