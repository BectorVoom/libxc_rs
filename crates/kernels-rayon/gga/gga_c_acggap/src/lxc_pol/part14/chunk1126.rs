//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1126/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1126(t5755: f64, t8511: f64, t1839: f64, t372: f64, t1181: f64, t2068: f64, t604: f64, t6283: f64, t7332: f64, t6255: f64, t7561: f64, t6260: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39594 = t8511 * t5755;
    let t39596 = t1839 * t372;
    let t39599 = t2068 * t1181 * t604 * t39596;
    let t39601 = t7332 * t6283;
    let t39605 = t7561 * t6255;
    let t39607 = t7561 * t6260;
    (t39594, t39596, t39599, t39601, t39605, t39607)
}
