//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 965/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk965(t29542: f64, t29545: f64, t29548: f64, t29551: f64, t29554: f64, t29556: f64, t29558: f64, t29562: f64, t29565: f64, t29567: f64, t29569: f64, t29573: f64, t29576: f64, t29578: f64, t29581: f64) -> f64 {
    let t30099 = -0.62499999999999999999e-1_f64 * t29542 + 0.10252083333333333334e1_f64 * t29545 + 0.40468749999999999999e-1_f64 * t29548 + 0.5625e0_f64 * t29551 - 0.13489583333333333333e-1_f64 * t29554 + 0.5625e0_f64 * t29556 - 0.28125e0_f64 * t29558 - 0.13489583333333333333e-1_f64 * t29562 - 0.13669444444444444444e1_f64 * t29565 - 0.40468749999999999999e-1_f64 * t29567 + 0.1875e0_f64 * t29569 + 0.625e-1_f64 * t29573 + 0.75e0_f64 * t29576 + 0.303515625e-1_f64 * t29578 + 0.21583333333333333333e0_f64 * t29581;
    t30099
}
