//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 813/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk813<F: Float>(t12711: F, t12716: F, t12720: F, t12726: F, t12730: F, t12734: F, t12739: F, t12743: F, t12748: F, t12752: F, t12756: F, t12759: F, t12763: F, t12767: F, t12771: F, t1901: F, t446: F) -> F {
    let t12774 = -F::new(4.0) / F::new(9.0) * t1901 * t12711 + F::new(4.0) / F::new(27.0) * t1901 * t12716 - F::new(2.0) / F::new(27.0) * t1901 * t12720 - F::new(10.0) / F::new(81.0) * t1901 * t12726 + t1901 * t12730 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t1901 * t12734 + t1901 * t12739 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t1901 * t12743 + F::new(4.0) / F::new(9.0) * t1901 * t12748 + F::new(4.0) / F::new(27.0) * t12752 + t446 * t12756 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t12759 - t446 * t12763 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t12767 - t446 * t12771 / F::new(9.0);
    t12774
}
