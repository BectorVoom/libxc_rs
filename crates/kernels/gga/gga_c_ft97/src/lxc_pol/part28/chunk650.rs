//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 650/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk650<F: Float>(t26356: F, t379: F, t1902: F, t6466: F, t8392: F, t1901: F, t26319: F, t26322: F, t26326: F, t26330: F, t26334: F, t26337: F, t26340: F, t26343: F, t26346: F, t26350: F, t26353: F, t3281: F, t446: F) -> (F, F) {
    let t26357 = t26356 * t379;
    let t26358 = t1902 * t26357;
    let t26361 = t8392 * t6466;
    let t26363 = -t1901 * t26319 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t26322 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t26326 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t26330 - t446 * t26334 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t26337 - t446 * t26340 / F::cast_from(3.0_f64) + t1901 * t26343 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t26346 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t26350 + t1901 * t26353 / F::cast_from(9.0_f64) + t1901 * t26358 / F::cast_from(9.0_f64) - t26361 / F::cast_from(27.0_f64);
    (t26357, t26363)
}
