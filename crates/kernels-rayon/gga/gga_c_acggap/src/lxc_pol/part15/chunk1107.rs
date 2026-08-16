//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1107/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1107(t368: f64, t5655: f64, t1980: f64, t30058: f64, t1165: f64, t2068: f64, t25727: f64, t7351: f64, t7337: f64, t8480: f64, t8774: f64, t5727: f64, t7647: f64) -> (f64, f64, f64, f64, f64) {
    let t39120 = t368 * t5655;
    let t39122 = t1980 * t30058 * t39120;
    let t39131 = t2068 * t1165 * t7351 * t25727;
    let t39134 = t7337 * t8480 * t8774;
    let t39136 = t7647 * t5727;
    (t39120, t39122, t39131, t39134, t39136)
}
