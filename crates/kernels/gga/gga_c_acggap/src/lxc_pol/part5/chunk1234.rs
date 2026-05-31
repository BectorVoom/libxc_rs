//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1234/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1234<F: Float>(t13748: F, t13750: F, t13754: F, t13771: F, t13814: F, t16233: F, t16238: F, t16241: F, t16244: F, t16249: F, t16253: F, t16255: F, t16264: F, t16274: F) -> F {
    let t22590 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16233 - t16238 / F::cast_from(2.0_f64) + t16241 / F::cast_from(3.0_f64) + t16244 / F::cast_from(6.0_f64) - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t16249 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t16253 + F::cast_from(4.0_f64) * t16255 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16264 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t16274 - t13814 + F::cast_from(140.0_f64) / F::cast_from(27.0_f64) * t13748 + F::cast_from(14.0_f64) / F::cast_from(9.0_f64) * t13750 - F::cast_from(7.0_f64) / F::cast_from(9.0_f64) * t13754 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t13771;
    t22590
}
