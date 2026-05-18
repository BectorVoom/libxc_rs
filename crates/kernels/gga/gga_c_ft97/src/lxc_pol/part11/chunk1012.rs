//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1012/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1012<F: Float>(t12968: F, t144: F, t160: F, t1643: F, t1647: F, t167: F, t1901: F, t2190: F, t2205: F, t379: F, t38071: F, t40594: F, t41246: F, t41251: F, t41262: F, t41264: F, t41267: F, t41269: F, t446: F, t558: F, t569: F, t604: F, t616: F, t7966: F, t9017: F, t9144: F, t9316: F) -> F {
    let t41278 = -F::new(8.0) / F::new(3.0) * t446 * t569 * t616 * t7966 - F::new(8.0) / F::new(3.0) * t446 * t2205 * t167 * t38071 - F::new(8.0) / F::new(3.0) * t41246 + F::new(8.0) * t446 * t144 * t40594 + F::new(8.0) / F::new(3.0) * t1901 * t41251 * t160 * t9017 * t379 - F::new(8.0) * t1901 * t12968 * t604 * t558 * t9316 - F::new(8.0) / F::new(9.0) * t41262 - F::new(16.0) / F::new(9.0) * t41264 - F::new(4.0) / F::new(9.0) * t41267 - F::new(8.0) / F::new(9.0) * t1901 * t41269 * t1643 * t2190 + F::new(8.0) / F::new(3.0) * t1901 * t9144 * t1647 * t2190;
    t41278
}
