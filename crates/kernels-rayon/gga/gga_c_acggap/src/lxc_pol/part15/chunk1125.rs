//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1125/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1125(t7413: f64, t8480: f64, t8947: f64, t1181: f64, t2068: f64, t26108: f64, t604: f64, t25732: f64, t142: f64, t6379: f64, t8806: f64, t6383: f64) -> (f64, f64, f64, f64, f64) {
    let t39454 = t7413 * t8480 * t8947;
    let t39458 = t2068 * t1181 * t604 * t26108;
    let t39462 = t2068 * t1181 * t604 * t25732;
    let t39465 = t8806 * t142 * t6379;
    let t39468 = t8806 * t142 * t6383;
    (t39454, t39458, t39462, t39465, t39468)
}
