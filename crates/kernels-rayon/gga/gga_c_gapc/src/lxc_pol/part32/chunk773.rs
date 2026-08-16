//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 773/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk773(t1448: f64, t3116: f64, t3115: f64, t3064: f64, t3122: f64, t3121: f64, t1625: f64, t1720: f64, t8987: f64, t197: f64, t4991: f64, t1022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9047 = t1448 * t3116;
    let t9048 = t3115 * t9047;
    let t9050 = t3064 * t3122;
    let t9051 = t3121 * t9050;
    let t9053 = t1720 * t1625;
    let t9054 = t8987 * t9053;
    let t9056 = t197 * t4991;
    let t9057 = t1022 * t9056;
    (t9048, t9050, t9051, t9053, t9054, t9057)
}
