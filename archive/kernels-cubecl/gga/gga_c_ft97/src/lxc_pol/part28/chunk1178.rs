//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1178/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1178<F: Float>(t165: F, t34918: F, t1349: F, t34970: F, t376: F, t35011: F, t34979: F, t104289: F, t138433: F, t138677: F, t138681: F, t138705: F, t1969: F, t24080: F, t26791: F, t27411: F, t28: F, t33000: F, t3424: F, t34961: F, t379: F, t5772: F, t5778: F, t614: F, t95403: F) -> F {
    let t149347 = t34918 * t165;
    let t149357 = t1349 * t376 * t34970;
    let t149360 = t1349 * t376 * t35011;
    let t149363 = t1349 * t376 * t34979;
    let t149369 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t28 * t26791 * t33000 - t138677 / F::cast_from(18.0_f64) - F::cast_from(24.0_f64) * t95403 * t27411 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t28 * t5778 * t104289 - t5772 * t1969 * t149347 * t379 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5772 * t24080 * t138433 * t3424 - t149357 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t149360 + t138681 + t138705 - t149363 / F::cast_from(9.0_f64) + t1349 * t28 * t34961 * t614 / F::cast_from(6.0_f64);
    t149369
}
