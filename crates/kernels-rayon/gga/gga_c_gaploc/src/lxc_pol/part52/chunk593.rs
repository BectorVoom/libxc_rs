//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 593/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk593(t10161: f64, t10164: f64, t10168: f64, t10175: f64, t1063: f64, t11154: f64, t11157: f64, t11160: f64, t11163: f64, t11202: f64, t11240: f64, t11284: f64, t2268: f64, t3519: f64, t3532: f64, t380: f64) -> f64 {
    let t11286 = 0.37940008847568199465e-1_f64 * t380 * t3532 + 0.37940008847568199465e-1_f64 * t380 * t3519 - 0.47425011059460249332e-2_f64 * t10161 + 0.47425011059460249332e-2_f64 * t10164 - 0.142275033178380748e-1_f64 * t10168 + 0.63233348079280332443e-2_f64 * t10175 - 0.28455006635676149599e-1_f64 * t1063 * t11154 + 0.56910013271352299198e-1_f64 * t2268 * t11157 + 0.28455006635676149599e-1_f64 * t2268 * t11160 - 0.28455006635676149599e-1_f64 * t1063 * t11163 + t11202 + t11240 + t11284;
    t11286
}
