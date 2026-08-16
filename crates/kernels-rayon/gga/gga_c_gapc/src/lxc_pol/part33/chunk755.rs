//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 755/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk755(t103: f64, t1552: f64, t1039: f64, t1035: f64, t3075: f64, t4925: f64, t3073: f64, t505: f64, t674: f64, t3143: f64, t3139: f64, t3060: f64, t3120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t8876 = t103 * t1552;
    let t8877 = t8876 * t1039;
    let t8878 = t1035 * t8877;
    let t8880 = t4925 * t3075;
    let t8881 = t3073 * t8880;
    let t8884 = pi * t505 * t674;
    let t8885 = t8884 * t3143;
    let t8886 = t3139 * t8885;
    let t8888 = t3060 * t3120;
    (t8877, t8878, t8880, t8881, t8884, t8885, t8886, t8888)
}
