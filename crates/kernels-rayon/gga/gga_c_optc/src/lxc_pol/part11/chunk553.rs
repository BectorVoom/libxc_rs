//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 553/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk553(t1540: f64, t2586: f64, t1170: f64, t1528: f64, t2367: f64, t1150: f64, t1129: f64, t1545: f64, t1157: f64, t1533: f64, t1567: f64, t176: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4509 = t2586 * t1540;
    let t4510 = t1170 * t4509;
    let t4512 = t2367 * t1528;
    let t4513 = t1150 * t4512;
    let t4515 = t1545 * t1129;
    let t4517 = t1533 * t1157;
    let t4535 = t176 * t1567;
    (t4509, t4510, t4512, t4513, t4515, t4517, t4535)
}
