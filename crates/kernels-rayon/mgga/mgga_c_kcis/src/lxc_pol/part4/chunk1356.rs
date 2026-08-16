//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1356/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1356(t1552: f64, t6020: f64, t1542: f64, t4291: f64, t5905: f64, t16673: f64, t4293: f64, t4292: f64, t1466: f64, t5997: f64, t1535: f64, t1489: f64, t5875: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t17441 = t6020 * t1552;
    let t17443 = t1542 * t4291;
    let t17444 = t17443 * t5905;
    let t17446 = t4293 * t16673;
    let t17447 = t4292 * t17446;
    let t17449 = t5997 * t1466;
    let t17450 = t17449 * sigma2;
    let t17451 = t17450 * t1535;
    let t17453 = t5875 * t1489;
    (t17441, t17444, t17447, t17451, t17453)
}
