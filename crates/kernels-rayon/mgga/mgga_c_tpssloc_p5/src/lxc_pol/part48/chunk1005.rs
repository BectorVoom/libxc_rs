//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1005/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1005(t2314: f64, t31772: f64, t1874: f64, t91857: f64, t26977: f64, t6525: f64, t22585: f64, t8607: f64, t31304: f64, t7000: f64, t6997: f64, t649: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115704 = 4.0_f64 * t2314 * t31772;
    let t115708 = 2.0_f64 * t91857 * t1874;
    let t115712 = 4.0_f64 * t26977 * t6525;
    let t115716 = 3.0_f64 * t8607 * t22585;
    let t115718 = 2.0_f64 * t31304 * t7000;
    let t115721 = 2.0_f64 * t31304 * t6997;
    let t115723 = t649 * t6534;
    (t115704, t115708, t115712, t115716, t115718, t115721, t115723)
}
