//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 974/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk974(t39123: f64, t959: f64, t13847: f64, t2684: f64, t7354: f64, t13872: f64, t2194: f64, t47311: f64, t568: f64, t808: f64, t813: f64, t13883: f64, t1589: f64, t797: f64) -> (f64, f64, f64, f64, f64) {
    let t47381 = t39123 * t959;
    let t47389 = t2684 * t7354 * t13847;
    let t47408 = 0.23005755572352449806e1_f64 * t2194 * t13872;
    let t47412 = 0.23005755572352449806e1_f64 * t813 * t568 * t808 * t47311;
    let t47415 = 0.23833659967900284446e0_f64 * t797 * t1589 * t13883;
    (t47381, t47389, t47408, t47412, t47415)
}
