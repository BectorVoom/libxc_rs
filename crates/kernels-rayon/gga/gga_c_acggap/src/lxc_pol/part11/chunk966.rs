//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 966/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk966(t1960: f64, t3889: f64, t2137: f64, t7930: f64, t322: f64, t7932: f64, t7934: f64, t309: f64, t955: f64, t7963: f64, t609: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32001 = t1960 * t3889;
    let t32003 = t2137 * t7930;
    let t32004 = t7932 * t322;
    let t32006 = t32003 * t32004 * t7934;
    let t32010 = t955 * t309;
    let t32012 = t7963 * t7932 * t32010;
    let t32029 = t848 * t609;
    (t32001, t32003, t32004, t32006, t32012, t32029)
}
