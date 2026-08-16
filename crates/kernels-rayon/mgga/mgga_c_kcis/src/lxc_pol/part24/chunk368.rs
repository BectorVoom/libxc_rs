//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 368/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk368(t1240: f64, t2192: f64, t209: f64, t9: f64, t1268: f64, t287: f64, t421: f64) -> (f64, f64, f64, f64) {
    let t2193 = t1240 * t2192;
    let t2194 = t209 * t9;
    let t2196 = t287 * t421 * t1268;
    let t2197 = t2194 * t2196;
    (t2193, t2194, t2196, t2197)
}
