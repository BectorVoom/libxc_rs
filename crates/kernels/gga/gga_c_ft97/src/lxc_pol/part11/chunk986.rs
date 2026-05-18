//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 986/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk986<F: Float>(t2180: F, t40524: F, t39673: F, t39662: F, t39666: F, t39670: F, t39679: F, t39681: F, t39683: F, t39685: F, t39687: F, t39689: F, t39691: F, t39696: F, t39700: F) -> (F, F) {
    let t40525 = t40524 * t2180;
    let t40530 = F::new(140.0) / F::new(243.0) * t39673;
    let t40540 = F::new(4.0) / F::new(9.0) * t39662 - F::new(4.0) / F::new(3.0) * t39666 - F::new(4.0) / F::new(3.0) * t39670 + t40530 - F::new(10.0) / F::new(27.0) * t39679 + F::new(4.0) / F::new(9.0) * t39681 - F::new(2.0) / F::new(9.0) * t39683 - F::new(4.0) / F::new(9.0) * t39685 + F::new(4.0) / F::new(27.0) * t39687 - F::new(4.0) / F::new(27.0) * t39689 + F::new(4.0) / F::new(9.0) * t39691 - F::new(4.0) / F::new(9.0) * t39696 - F::new(8.0) / F::new(9.0) * t39700;
    (t40525, t40540)
}
