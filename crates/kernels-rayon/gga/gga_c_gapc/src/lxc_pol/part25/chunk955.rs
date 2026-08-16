//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 955/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk955(t3563: f64, t883: f64, t1117: f64, t2468: f64, t10103: f64, t10106: f64, t10108: f64, t10111: f64, t10115: f64, t10118: f64, t10120: f64, t10126: f64, t10128: f64, t10131: f64, t10134: f64) -> (f64, f64, f64) {
    let t11043 = t3563 * t883;
    let t11046 = t1117 * t2468;
    let t11060 = -0.54715885245250729722e-5_f64 * t10103 + 0.26446011201871186032e-4_f64 * t10106 + 0.25051693218177510181e-2_f64 * t10108 + 0.23485962392041415794e-5_f64 * t10111 + 0.3757753982726626527e-4_f64 * t10115 + 0.54715885245250729722e-5_f64 * t10118 + 0.18968173551686919637e-3_f64 * t10120 + 0.39597758471766536049e-5_f64 * t10126 + 0.29230628793134746097e-4_f64 * t10128 - 0.56366309740899397906e-3_f64 * t10131 - 0.3757753982726626527e-4_f64 * t10134;
    (t11043, t11046, t11060)
}
