//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1003/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1003(t531: f64, t8639: f64, t1983: f64, t22596: f64, t115227: f64, t115229: f64, t115231: f64, t115233: f64, t115238: f64, t115241: f64, t115245: f64, t115249: f64, t115251: f64, t115254: f64, t115256: f64, t115261: f64, t1976: f64, t2036: f64, t2040: f64, t2075: f64, t22600: f64, t23829: f64, t23909: f64, t23917: f64, t6517: f64, t652: f64, t672: f64, t83935: f64) -> f64 {
    let t115262 = t531 * t8639;
    let t115265 = 6.0_f64 * t1983 * t115262 * t22596;
    let t115267 = -2.0_f64 * t1976 * t23917 * t652 - 4.0_f64 * t115241 * t672 - t2036 * t23829 - 2.0_f64 * t2040 * t83935 - 2.0_f64 * t2075 * t22600 - 2.0_f64 * t23909 * t6517 + t115227 - t115229 - t115231 - t115233 + t115238 + t115245 - t115249 - t115251 - t115254 - t115256 - t115261 + t115265;
    t115267
}
