//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1017/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1017<F: Float>(t2299: F, t13255: F, t1580: F, t184: F, t21: F, t2240: F, t2300: F, t2305: F, t2309: F, t3664: F, t40590: F, t41364: F, t5: F, t623: F, t648: F, t8614: F, t8722: F, t8724: F, t8738: F, t8739: F, t8744: F, t8751: F, t8754: F) -> F {
    let t41382 = t2299 * t2299;
    let t41399 = F::cast_from(3.0_f64) * t8614 * t2309 + t2240 * t8724 + t5 * (t40590 + t41364) * t184 * t21 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t2305 * t21 * t2299 + F::cast_from(3.0_f64) * t2240 * t8751 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t2305 * t1580 + t623 * t8722 * t648 * t3664 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t623 * t41382 * t184 * t21 + F::cast_from(3.0_f64) * t2240 * t8739 + F::cast_from(3.0_f64) * t623 * t8738 * t13255 + F::cast_from(3.0_f64) * t2240 * t8744 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t2300 * t1580 + F::cast_from(3.0_f64) * t2240 * t8754;
    t41399
}
