//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 965/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk965<F: Float>(t29542: F, t29545: F, t29548: F, t29551: F, t29554: F, t29556: F, t29558: F, t29562: F, t29565: F, t29567: F, t29569: F, t29573: F, t29576: F, t29578: F, t29581: F) -> F {
    let t30099 = -F::cast_from(0.62499999999999999999e-1_f64) * t29542 + F::cast_from(0.10252083333333333334e1_f64) * t29545 + F::cast_from(0.40468749999999999999e-1_f64) * t29548 + F::cast_from(0.5625e0_f64) * t29551 - F::cast_from(0.13489583333333333333e-1_f64) * t29554 + F::cast_from(0.5625e0_f64) * t29556 - F::cast_from(0.28125e0_f64) * t29558 - F::cast_from(0.13489583333333333333e-1_f64) * t29562 - F::cast_from(0.13669444444444444444e1_f64) * t29565 - F::cast_from(0.40468749999999999999e-1_f64) * t29567 + F::cast_from(0.1875e0_f64) * t29569 + F::cast_from(0.625e-1_f64) * t29573 + F::cast_from(0.75e0_f64) * t29576 + F::cast_from(0.303515625e-1_f64) * t29578 + F::cast_from(0.21583333333333333333e0_f64) * t29581;
    t30099
}
