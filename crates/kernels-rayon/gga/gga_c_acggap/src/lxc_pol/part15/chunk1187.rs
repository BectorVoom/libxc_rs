//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1187/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1187(t1782: f64, t1992: f64, t2095: f64, t7426: f64, t8480: f64, t8605: f64, t4680: f64, t7564: f64, t9607: f64, t1181: f64, t5819: f64, t7351: f64) -> (f64, f64, f64, f64) {
    let t40546 = t2095 * t1992 * t1782;
    let t40549 = t7426 * t8480 * t8605;
    let t40553 = t7564 * t4680 * t9607;
    let t40557 = t7564 * t1181 * t7351 * t5819;
    (t40546, t40549, t40553, t40557)
}
