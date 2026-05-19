//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 460/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk460<F: Float>(t140: F, t1722: F, t1733: F, t2066: F, t3083: F, t3086: F, t3090: F, t3093: F, t3097: F, t550: F, t133: F, t1010: F, t1015: F, t2001: F, t3348: F, t3350: F, t3356: F, t3381: F, t3384: F, t3387: F, t3392: F, t3394: F, t399: F) -> (F, F) {
    let t141 = F::new(0.1e-59) < t140;
    let t3404 = -F::cast_from(0.44452000728395061731e-1_f64) * t1722 - t2066 + F::cast_from(0.55565000910493827163e-2_f64) * t1733 - F::cast_from(0.44452000728395061731e-1_f64) * t3083 + F::cast_from(0.55565000910493827163e-2_f64) * t3086 + F::cast_from(0.22226000364197530865e-1_f64) * t3090 - F::cast_from(0.33339000546296296298e-1_f64) * t3093 + F::cast_from(0.33339000546296296298e-1_f64) * t3097;
    let t3405 = t550 * t3404;
    let t3406 = t133 * t3405;
    let t3408 = piecewise3::<F>(t141, F::new(2.0) * t3348 - F::cast_from(0.1208182677680765956e1_f64) * t3350 * t399 + F::cast_from(0.1208182677680765956e1_f64) * t1010 * t399 - F::new(2.0) * t2001 * t3356 + F::new(2.0) * t3381 - F::new(2.0) * t2001 * t3384 + F::cast_from(0.60409133884038297798e0_f64) * t3387 * t399 - F::cast_from(0.60409133884038297798e0_f64) * t1015 * t399 + F::new(2.0) * t3392 * t3394 - t3406, F::new(0.0));
    (t3404, t3408)
}
