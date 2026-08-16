//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1025/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1025(t11041: f64, t11043: f64, t11046: f64, t11049: f64, t11050: f64, t11056: f64, t11059: f64, t11063: f64, t11067: f64, t11071: f64, t11072: f64, t11075: f64, t11080: f64, t11084: f64, t1991: f64, t317: f64, t3465: f64, t784: f64, t797: f64, t813: f64) -> f64 {
    let t11087 = -t11041 + t11043 + t11046 + t11049 + 0.51123901271894332902e0_f64 * t1991 * t11050 - t11056 - t11059 + t11063 - t11067 + t11071 - 0.30674340763136599741e1_f64 * t813 * t11072 - 0.23833659967900284446e0_f64 * t797 * t11075 + 0.23833659967900284446e0_f64 * t3465 * t784 + 0.35750489951850426669e0_f64 * t11080 * t317 + 0.35750489951850426669e0_f64 * t11084 * t317;
    t11087
}
