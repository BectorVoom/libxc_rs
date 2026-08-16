//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 906/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk906(t14012: f64, t762: f64, t242: f64, t1882: f64, t3861: f64, t3866: f64, t1175: f64, t2413: f64, t724: f64, t2405: f64, t2594: f64, t4005: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14013 = t762 * t14012;
    let t14014 = t242 * t14013;
    let t14018 = 2.0_f64 / 9.0_f64 * t1882 * t3861;
    let t14020 = 4.0_f64 / 9.0_f64 * t1882 * t3866;
    let t14022 = t724 * t1175 * t2413;
    let t14026 = t2594 * t1175 * t2405;
    let t14030 = t724 * t4005 * t684;
    (t14013, t14014, t14018, t14020, t14022, t14026, t14030)
}
