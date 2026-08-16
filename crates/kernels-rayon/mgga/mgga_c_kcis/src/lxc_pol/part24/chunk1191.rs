//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1191/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1191(t1014: f64, t27928: f64, t26717: f64, t8030: f64, t26854: f64, t27931: f64, t27964: f64, t7699: f64, t27851: f64, t1009: f64, t14400: f64, t8048: f64, t9562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96005 = t1014 * t27928;
    let t96010 = 0.46336805555555555556e-3_f64 * t8030 * t26717;
    let t96015 = t8030 * t26854;
    let t96018 = t1014 * t27931;
    let t96019 = 0.33163888888888888888e-2_f64 * t96018;
    let t96026 = 0.12356481481481481482e-2_f64 * t27964 * t7699;
    let t96068 = t1014 * t27851;
    let t96108 = t14400 * t1009;
    let t96121 = t9562 * t8048;
    (t96005, t96010, t96015, t96018, t96019, t96026, t96068, t96108, t96121)
}
