//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 927/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk927(t2132: f64, t322: f64, t7896: f64, t7979: f64, t2159: f64, t7924: f64, t1960: f64, t3889: f64, t2137: f64, t7930: f64, t7932: f64, t609: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31976 = t7896 * t2132 * t7979 * t322;
    let t31978 = t7924 * t2159;
    let t32001 = t1960 * t3889;
    let t32003 = t2137 * t7930;
    let t32004 = t7932 * t322;
    let t32029 = t848 * t609;
    (t31976, t31978, t32001, t32003, t32004, t32029)
}
