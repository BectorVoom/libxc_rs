//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1420/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1420(t1226: f64, t4965: f64, t11697: f64, t4953: f64, t3577: f64, t1229: f64, t3242: f64, t13969: f64, t4979: f64, t3506: f64, t4973: f64, t1227: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15594 = t4965 * t1226;
    let t15608 = t11697 * t4953;
    let t15610 = t3577 * t15608 / 3456.0_f64;
    let t15615 = t1229 * t3242;
    let t15640 = t13969 * t4979;
    let t15642 = t3506 * t15640 / 1152.0_f64;
    let t15643 = t13969 * t4973;
    let t15645 = t1227 * t15643 / 1728.0_f64;
    (t15594, t15608, t15610, t15615, t15640, t15642, t15643, t15645)
}
