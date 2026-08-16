//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1116/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1116(t233: f64, t29232: f64, t1658: f64, t8121: f64, t2167: f64, t6290: f64, t10819: f64, t1259: f64, t3530: f64, t3622: f64, t779: f64, t9274: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29233 = t233 * t29232;
    let t29234 = t29233 / 16.0_f64;
    let t29235 = t1658 * t8121;
    let t29236 = t233 * t29235;
    let t29237 = t29236 / 8.0_f64;
    let t29238 = t6290 * t2167;
    let t30045 = t1259 * t10819;
    let t30066 = t3530 * t3622;
    let t31271 = t779 * t9274;
    (t29234, t29235, t29237, t29238, t30045, t30066, t31271)
}
