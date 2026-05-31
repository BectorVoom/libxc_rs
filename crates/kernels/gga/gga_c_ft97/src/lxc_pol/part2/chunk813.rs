//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 813/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk813<F: Float>(t12711: F, t12716: F, t12720: F, t12726: F, t12730: F, t12734: F, t12739: F, t12743: F, t12748: F, t12752: F, t12756: F, t12759: F, t12763: F, t12767: F, t12771: F, t1901: F, t446: F) -> F {
    let t12774 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t12711 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t12716 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t12720 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1901 * t12726 + t1901 * t12730 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t12734 + t1901 * t12739 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t12743 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t12748 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12752 + t446 * t12756 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t12759 - t446 * t12763 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t446 * t12767 - t446 * t12771 / F::cast_from(9.0_f64);
    t12774
}
