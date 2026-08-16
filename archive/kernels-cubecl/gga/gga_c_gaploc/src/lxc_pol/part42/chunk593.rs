//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 593/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk593<F: Float>(t10161: F, t10164: F, t10168: F, t10175: F, t1063: F, t11154: F, t11157: F, t11160: F, t11163: F, t11202: F, t11240: F, t11284: F, t2268: F, t3519: F, t3532: F, t380: F) -> F {
    let t11286 = F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3532 + F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3519 - F::cast_from(0.47425011059460249332e-2_f64) * t10161 + F::cast_from(0.47425011059460249332e-2_f64) * t10164 - F::cast_from(0.142275033178380748e-1_f64) * t10168 + F::cast_from(0.63233348079280332443e-2_f64) * t10175 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t11154 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t11157 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t11160 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t11163 + t11202 + t11240 + t11284;
    t11286
}
