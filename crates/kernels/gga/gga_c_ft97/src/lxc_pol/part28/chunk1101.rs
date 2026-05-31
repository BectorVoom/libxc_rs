//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1101/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1101<F: Float>(t26564: F, t7309: F, t26574: F, t1349: F, t138549: F, t139418: F, t26540: F, t26553: F, t26771: F, t28: F, t32686: F, t32738: F, t32998: F, t35007: F, t5781: F, t5843: F, t6580: F, t6587: F, t6622: F, t6723: F) -> F {
    let t147198 = t7309 * t26564;
    let t147216 = t7309 * t26574;
    let t147224 = -t147198 / F::cast_from(18.0_f64) + t1349 * t28 * t5843 * t6723 / F::cast_from(3.0_f64) - t7309 * t26553 / F::cast_from(3.0_f64) + t138549 / F::cast_from(9.0_f64) + t6580 * t32998 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6580 * t32738 - t35007 * t5781 / F::cast_from(3.0_f64) - t7309 * t26540 / F::cast_from(3.0_f64) + t7309 * t26771 / F::cast_from(6.0_f64) - t147216 / F::cast_from(18.0_f64) + t32686 * t6622 / F::cast_from(6.0_f64) - t1349 * t28 * t139418 * t6587 / F::cast_from(3.0_f64);
    t147224
}
