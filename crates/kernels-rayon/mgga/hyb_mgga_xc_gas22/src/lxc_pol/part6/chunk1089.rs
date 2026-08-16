//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1089/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1089(t10589: f64, t10615: f64, t810: f64, t788: f64, t4143: f64, t6574: f64, t3443: f64, t8769: f64, t2311: f64, t4193: f64, t3444: f64, t4180: f64, t6666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10616 = t10589 + t10615;
    let t10617 = t10616 * t810;
    let t10619 = 1.0_f64 * t788 * t10617;
    let t10621 = 0.16081979498692535067e2_f64 * t6574 * t4143;
    let t10622 = t3443 * t8769;
    let t10625 = t2311 * t4193;
    let t10626 = t10625 * t3444;
    let t10629 = t6666 * t4180;
    (t10616, t10617, t10619, t10621, t10622, t10625, t10626, t10629)
}
