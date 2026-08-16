//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 823/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk823<F: Float>(t1039: F, t2087: F, t91: F, t9252: F, t2086: F, t3526: F, t590: F, t2120: F, t3491: F, t12574: F, t12577: F, t12580: F, t12584: F, t12589: F, t12592: F, t12918: F, t9062: F) -> (F, F, F, F) {
    let t12921 = t91 * t9252 * t1039 * t2087;
    let t12923 = t2086 * t3526;
    let t12925 = t91 * t12923 * t590;
    let t12928 = t91 * t3491 * t2120;
    let t12937 = -t12918 + t12921 / F::cast_from(8.0_f64) - t12925 / F::cast_from(6.0_f64) - t12928 / F::cast_from(12.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12574 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12577 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t12580 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12584 - F::cast_from(2.0_f64) * t12589 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12592 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9062;
    (t12921, t12925, t12928, t12937)
}
