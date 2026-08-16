//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1181/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1181<F: Float>(t1349: F, t139192: F, t147930: F, t148166: F, t148234: F, t149093: F, t149120: F, t149141: F, t24080: F, t26568: F, t28: F, t34967: F, t35016: F, t558: F, t5766: F, t5772: F, t5778: F, t5973: F, t6616: F, t6723: F) -> F {
    let t149458 = -t5766 * t35016 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5772 * t24080 * t26568 + t139192 / F::cast_from(54.0_f64) + t5766 * t34967 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t28 * t5778 * t6723 * t558 + F::cast_from(8.0_f64) * t147930 + F::cast_from(8.0_f64) * t148166 + F::cast_from(8.0_f64) * t149120 + t1349 * t28 * t6616 * t5973 / F::cast_from(3.0_f64) + F::cast_from(8.0_f64) * t149141 - F::cast_from(12.0_f64) * t148234 + F::cast_from(4.0_f64) * t149093;
    t149458
}
