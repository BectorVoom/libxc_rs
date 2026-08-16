//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 764/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk764(t157: f64, t609: f64, t929: f64, t2152: f64, t2124: f64, t310: f64, t611: f64, t848: f64, t315: f64, t7941: f64) -> (f64, f64, f64, f64) {
    let t7953 = t609 * t929 * t157;
    let t7954 = t2152 * t7953;
    let t7957 = t310 * t2124;
    let t7962 = 0.65854491829355115987e0_f64 * t848 * t611;
    let t7963 = t315 * t7941;
    (t7954, t7957, t7962, t7963)
}
