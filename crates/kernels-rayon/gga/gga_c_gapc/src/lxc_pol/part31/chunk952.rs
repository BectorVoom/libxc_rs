//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 952/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk952(t9857: f64, t9860: f64, t9866: f64, t9869: f64, t9872: f64, t9874: f64, t9876: f64, t9878: f64, t9881: f64, t9883: f64, t9885: f64, t9887: f64, t9889: f64) -> f64 {
    let t10975 = -0.34752370105806885418e-3_f64 * t9857 + 0.51491428373437201896e-5_f64 * t9860 + 0.98478623777692089505e-7_f64 * t9866 + 0.34752370105806885418e-3_f64 * t9869 + 0.17376185052903442709e-3_f64 * t9872 + 0.4637672555408563478e-4_f64 * t9874 - 0.30353495895471971564e-6_f64 * t9876 + 0.53968515702149165441e-6_f64 * t9878 - 0.46497498276882732785e-5_f64 * t9881 + 0.43284943850479925795e-3_f64 * t9883 - 0.43284943850479925795e-3_f64 * t9885 - 0.41223756048076119805e-5_f64 * t9887 + 0.73295838253479341016e-5_f64 * t9889;
    t10975
}
