//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1204/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1204<F: Float>(t43538: F, t71276: F, t71298: F, t71305: F, t71319: F, t83728: F, t83770: F, t83772: F, t83781: F, t83789: F, t83792: F, t90326: F, t90330: F, t90335: F, t90468: F) -> F {
    let t91080 = F::new(40.0) / F::new(243.0) * t83728 - F::new(5.0) / F::new(16.0) * t90326 - t90330 / F::new(4.0) + F::new(16.0) / F::new(27.0) * t71276 + t43538 + F::new(8.0) / F::new(3.0) * t90335 + t90468 / F::new(6.0) + F::new(4.0) / F::new(9.0) * t83770 - F::new(8.0) / F::new(27.0) * t83772 + F::new(8.0) / F::new(9.0) * t83781 - F::new(8.0) / F::new(9.0) * t83789 + F::new(8.0) / F::new(3.0) * t83792 - F::new(16.0) / F::new(81.0) * t71298 + F::new(16.0) / F::new(27.0) * t71305 - F::new(8.0) / F::new(27.0) * t71319;
    t91080
}
