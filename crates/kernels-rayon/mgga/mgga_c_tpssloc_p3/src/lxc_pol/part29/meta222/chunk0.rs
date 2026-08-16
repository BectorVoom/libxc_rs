//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1058/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1058(t4964: f64, t68: f64, t484: f64, t1177: f64, t4729: f64, t1229: f64, t3247: f64, t3961: f64, t4582: f64, t1734: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4965 = t4964 * t68;
    let t4966 = t4965 * t484;
    let t4969 = t1177 * t4729;
    let t4972 = t1229 * t3247;
    let t4973 = t4972 * t3961;
    let t4974 = t4582 * t4973;
    let t4977 = t486 * t1734;
    (t4965, t4966, t4969, t4972, t4973, t4974, t4977)
}
