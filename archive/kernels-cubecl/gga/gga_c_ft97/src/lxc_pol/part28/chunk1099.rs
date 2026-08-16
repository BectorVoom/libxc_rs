//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1099/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1099<F: Float>(t1349: F, t34966: F, t376: F, t32706: F, t6580: F, t32699: F, t35007: F, t5769: F, t138480: F, t138493: F, t138521: F, t138524: F, t26533: F, t26538: F, t28: F, t32709: F, t35206: F, t39653: F, t5849: F, t609: F) -> F {
    let t147142 = t1349 * t376 * t34966;
    let t147144 = t6580 * t32706;
    let t147152 = t6580 * t32699;
    let t147154 = t35007 * t5769;
    let t147159 = -t138480 + t138493 / F::cast_from(27.0_f64) + t35007 * t5849 / F::cast_from(6.0_f64) - t1349 * t28 * t32709 * t26538 / F::cast_from(3.0_f64) - t147142 / F::cast_from(18.0_f64) - t147144 / F::cast_from(18.0_f64) - t1349 * t28 * t32709 * t26533 / F::cast_from(3.0_f64) - t138521 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t138524 - t147152 / F::cast_from(9.0_f64) - t147154 / F::cast_from(18.0_f64) + F::cast_from(48.0_f64) * t39653 * t35206 * t609;
    t147159
}
