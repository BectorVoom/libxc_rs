//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 860/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk860<F: Float>(t16287: F, t185: F, t108: F, t176: F, t1303: F, t13056: F, t13573: F, t13578: F, t16341: F, t16342: F, t16344: F, t16345: F, t203: F, t3308: F, t6480: F, t6484: F, t6816: F) -> (F, F) {
    let t16604 = t185 * t16287;
    let t16606 = t176 * t16604 * t108;
    let t16614 = t16341 + t16342 + t16344 + t6816 - t16345 + t16606 * t203 / F::new(2.0) - F::cast_from(0.77534644304710291488e-2_f64) * t3308 * t13056 * t1303 + F::new(3.0) * t13573 + F::new(3.0) / F::new(2.0) * t13578 - t6480 - t6484;
    (t16606, t16614)
}
