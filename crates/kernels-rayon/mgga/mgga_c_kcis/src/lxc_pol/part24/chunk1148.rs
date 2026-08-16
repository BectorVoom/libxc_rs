//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1148/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1148(t2398: f64, t7639: f64, t8759: f64, t26490: f64, t7633: f64, t26450: f64, t7647: f64, t26477: f64, t7636: f64, t92016: f64, t26501: f64, t2155: f64, t92055: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92066 = t8759 * t2398 * t7639;
    let t92068 = t7633 * t26490;
    let t92070 = t26450 * t7647;
    let t92072 = t26450 * t7639;
    let t92074 = t7633 * t26477;
    let t92076 = t7636 * t92016;
    let t92078 = t7633 * t26501;
    let t92080 = t2155 * t92055;
    (t92066, t92068, t92070, t92072, t92074, t92076, t92078, t92080)
}
