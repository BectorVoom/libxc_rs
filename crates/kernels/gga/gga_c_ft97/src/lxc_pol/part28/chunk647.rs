//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 647/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk647<F: Float>(t1901: F, t23176: F, t26276: F, t26280: F, t26284: F, t26288: F, t26291: F, t26293: F, t26295: F, t26297: F, t26301: F, t26303: F, t26306: F, t26309: F, t26312: F, t446: F) -> F {
    let t26315 = -t446 * t26276 / F::new(3.0) - t446 * t26280 / F::new(3.0) - t446 * t26284 / F::new(3.0) - t446 * t26288 / F::new(3.0) + t26291 / F::new(9.0) + t26293 / F::new(9.0) + t26295 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t1901 * t26297 - t23176 / F::new(9.0) - t26301 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t26303 - t1901 * t26306 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t26309 - F::new(2.0) / F::new(9.0) * t1901 * t26312;
    t26315
}
