//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 966/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk966(t31878: f64, t3453: f64, t309: f64, t945: f64, t1219: f64, t615: f64, t7911: f64, t2137: f64, t7930: f64, t955: f64, t7884: f64, t7941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31879 = t31878 * t3453;
    let t31935 = t945 * t309;
    let t31965 = t615 * t7911 * t1219;
    let t32003 = t2137 * t7930;
    let t32010 = t955 * t309;
    let t32041 = t7884 * t7941;
    (t31879, t31935, t31965, t32003, t32010, t32041)
}
