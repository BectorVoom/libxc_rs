//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 835/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk835<F: Float>(t34379: F, t8270: F, t1317: F, t28: F, t1800: F, t34384: F, t7211: F, t965: F, t469: F, t5665: F, t32435: F, t32440: F, t34373: F, t34377: F, t34382: F, t34387: F, t34391: F, t34395: F) -> (F, F, F, F, F, F, F, F) {
    let t34397 = t8270 * t34379;
    let t34399 = t1317 * t28 * t34397;
    let t34401 = t1800 * t34384;
    let t34403 = t1317 * t28 * t34401;
    let t34405 = t7211 * t965;
    let t34406 = t469 * t34405;
    let t34408 = t5665 * t28 * t34406;
    let t34410 = F::new(3.0) / F::new(2.0) * t34373 + t32435 + F::new(2.0) / F::new(3.0) * t34377 + F::new(4.0) * t34382 - F::new(2.0) * t34387 - t34391 / F::new(2.0) - t32440 - t34395 / F::new(3.0) - F::new(3.0) * t34399 + F::new(2.0) * t34403 + t34408 / F::new(4.0);
    (t34397, t34399, t34401, t34403, t34405, t34406, t34408, t34410)
}
