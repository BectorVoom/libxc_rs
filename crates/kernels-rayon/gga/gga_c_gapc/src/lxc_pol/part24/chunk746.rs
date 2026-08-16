//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 746/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk746(t3076: f64, t8869: f64, t3060: f64, t3138: f64, t3144: f64, t103: f64, t1552: f64, t1039: f64, t1035: f64, t3075: f64, t4925: f64, t3073: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8870 = t8869 * t3076;
    let t8872 = t3060 * t3138;
    let t8873 = t8872 * t3144;
    let t8876 = t103 * t1552;
    let t8877 = t8876 * t1039;
    let t8878 = t1035 * t8877;
    let t8880 = t4925 * t3075;
    let t8881 = t3073 * t8880;
    (t8870, t8873, t8877, t8878, t8880, t8881)
}
