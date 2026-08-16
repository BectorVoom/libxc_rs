//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1150/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1150(t13181: f64, t389: f64, t26938: f64, t8072: f64, t1096: f64, t5096: f64, t1021: f64, t5086: f64, t26929: f64, t380: f64, t5182: f64, t1189: f64, t5026: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28016 = t13181 * t389;
    let t28018 = t26938 * t8072;
    let t28020 = t1096 * t5096;
    let t28022 = t1021 * t5086;
    let t28024 = t380 * t26929;
    let t28025 = t28024 * t5182;
    let t28027 = t5026 * t1189;
    (t28016, t28018, t28020, t28022, t28024, t28025, t28027)
}
