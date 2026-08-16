//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 307/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk307<F: Float>(t140: F, t3404: F, t550: F, t133: F, t1010: F, t1015: F, t2001: F, t3348: F, t3350: F, t3356: F, t3381: F, t3384: F, t3387: F, t3392: F, t3394: F, t399: F) -> (F, F) {
    let t141 = F::cast_from(0.1e-59_f64) < t140;
    let t3405 = t550 * t3404;
    let t3406 = t133 * t3405;
    let t3408 = piecewise3::<F>(t141, F::cast_from(2.0_f64) * t3348 - F::cast_from(0.1208182677680765956e1_f64) * t3350 * t399 + F::cast_from(0.1208182677680765956e1_f64) * t1010 * t399 - F::cast_from(2.0_f64) * t2001 * t3356 + F::cast_from(2.0_f64) * t3381 - F::cast_from(2.0_f64) * t2001 * t3384 + F::cast_from(0.60409133884038297798e0_f64) * t3387 * t399 - F::cast_from(0.60409133884038297798e0_f64) * t1015 * t399 + F::cast_from(2.0_f64) * t3392 * t3394 - t3406, F::cast_from(0.0_f64));
    (t3405, t3408)
}
