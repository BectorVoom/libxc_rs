//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 654/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk654(t3822: f64, t576: f64, t3725: f64, t3730: f64, t3735: f64, t3740: f64, t338: f64, t1096: f64, t3565: f64, t1125: f64, t3265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3823 = t576 * t3822;
    let t3828 = 0.16414765573575218917e-4_f64 * t3725 - 0.23485962392041415794e-4_f64 * t3730 - 0.34197428278281706076e-6_f64 * t3735 + 0.14678726495025884871e-5_f64 * t3740;
    let t3829 = t3828 * t338;
    let t3830 = t3565 * t1096;
    let t3831 = t3265 * t1125;
    let t3832 = t1125 * t1096;
    (t3823, t3828, t3829, t3830, t3831, t3832)
}
