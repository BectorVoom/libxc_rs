//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 817/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk817<F: Float>(t342: F, t630: F, t7574: F, t231: F, t6260: F, t1466: F, t1526: F, t2: F, t2320: F, t34284: F, t34289: F, t34291: F, t34296: F, t343: F, t6335: F, t6340: F, t7570: F, t7571: F) -> (F, F, F) {
    let t34301 = t342 * t630 * t7574 / F::cast_from(12.0_f64);
    let t34305 = t231 * t6260;
    let t34310 = (-t34284 * t7571 / F::cast_from(6.0_f64) + t34289 + t1466 * t34291 / F::cast_from(18.0_f64) + t1466 * t6340 / F::cast_from(3.0_f64) - t7570 * t34296 / F::cast_from(6.0_f64) - t34301 - t1526 * t2320 * t6335 / F::cast_from(12.0_f64) - t342 * t343 * t34305 / F::cast_from(4.0_f64)) * t2;
    (t34301, t34305, t34310)
}
