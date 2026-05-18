//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1080/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1080<F: Float>(t160: F, t165: F, t515: F, t86595: F, t86598: F, t86601: F, t86604: F, t87088: F, t87091: F, t87093: F, t87095: F, t87097: F, t87163: F, t87175: F, t87187: F, t87200: F, t87214: F) -> F {
    let t87219 = F::new(16.0) * t86595 + F::new(12.0) * t86598 + F::new(48.0) * t86601 - F::new(72.0) * t86604 + F::new(2.0) * t87088 * t160 - F::new(12.0) * t87091 - F::new(48.0) * t87093 + F::new(48.0) * t87095 + F::new(24.0) * t87097 - F::new(2.0) * t87163 - t515 * (t87175 + t87187 + t87200 + t87214) * t165;
    t87219
}
