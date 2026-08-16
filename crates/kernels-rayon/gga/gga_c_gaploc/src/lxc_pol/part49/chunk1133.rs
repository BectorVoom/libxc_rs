//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1133/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1133(t41312: f64, t41316: f64, t13872: f64, t2194: f64, t47311: f64, t568: f64, t808: f64, t813: f64, t13883: f64, t1589: f64, t797: f64, t13880: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47405 = 0.63904876589867916128e-1_f64 * t41312;
    let t47406 = 0.63904876589867916128e-1_f64 * t41316;
    let t47408 = 0.23005755572352449806e1_f64 * t2194 * t13872;
    let t47412 = 0.23005755572352449806e1_f64 * t813 * t568 * t808 * t47311;
    let t47415 = 0.23833659967900284446e0_f64 * t797 * t1589 * t13883;
    let t47417 = 0.23833659967900284446e0_f64 * t13880 * t784;
    (t47405, t47406, t47408, t47412, t47415, t47417)
}
