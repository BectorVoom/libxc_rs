//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 808/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk808<F: Float>(t167: F, t9132: F, t12334: F, t12666: F, t12670: F, t12672: F, t12674: F, t12676: F, t12677: F, t12681: F, t12685: F, t12689: F, t12696: F, t12700: F, t1901: F, t446: F, t9090: F, t9097: F, t9106: F) -> F {
    let t12703 = t9132 * t167;
    let t12704 = t12703 * t12334;
    let t12707 = F::new(2.0) / F::new(3.0) * t446 * t12666 + t12670 + t12672 + t12674 + t12676 + F::new(2.0) / F::new(9.0) * t1901 * t12677 + F::new(2.0) / F::new(9.0) * t1901 * t12681 - t446 * t12685 / F::new(3.0) - t446 * t12689 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t9090 - F::new(2.0) / F::new(27.0) * t9097 + t9106 / F::new(9.0) - t446 * t12696 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t12700 - F::new(4.0) / F::new(9.0) * t1901 * t12704;
    t12707
}
