//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 816/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk816(t1212: f64, t4965: f64, t10409: f64, t446: f64, t10414: f64, t21181: f64, t2345: f64, t89: f64, t21196: f64, t2857: f64, t4973: f64, t2665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21958 = t4965 * t1212;
    let t21959 = t10409 * t21958;
    let t21960 = t446 * t21959;
    let t21962 = t10414 * t21181;
    let t21964 = t89 * t2345 * t21962;
    let t21966 = t2857 * t21196;
    let t21967 = t446 * t21966;
    let t21969 = t4973 * t1212;
    let t21970 = t2665 * t21969;
    (t21958, t21959, t21960, t21962, t21964, t21966, t21967, t21969, t21970)
}
