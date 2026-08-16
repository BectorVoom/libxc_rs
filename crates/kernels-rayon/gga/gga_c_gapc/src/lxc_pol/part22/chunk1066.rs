//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1066/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1066(t103: f64, t1431: f64, t1037: f64, t1403: f64, t102: f64, t1338: f64, t1946: f64, t4864: f64, t640: f64, t11589: f64, t567: f64, t4: f64, t4054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t13337 = t103 * t1431;
    let t13483 = t1037 * t1403;
    let t13537 = t1946 * t102 * t1338;
    let t13541 = t4864 * t640;
    let t13646 = t11589 * pi * t567;
    let t13654 = t4054 * t4;
    (t13337, t13483, t13537, t13541, t13646, t13654)
}
