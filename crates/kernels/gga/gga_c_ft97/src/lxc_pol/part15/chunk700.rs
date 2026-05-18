//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 700/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk700<F: Float>(t1902: F, t20291: F, t11846: F, t11883: F, t16255: F, t16296: F, t16298: F, t16300: F, t16302: F, t16490: F, t16539: F, t1901: F, t20276: F, t20281: F, t20284: F, t20288: F, t446: F, t8534: F) -> (F, F) {
    let t20292 = t1902 * t20291;
    let t20304 = -t446 * t20276 / F::new(3.0) + t446 * t20281 - F::new(2.0) / F::new(3.0) * t446 * t20284 + t1901 * t20288 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1901 * t20292 - F::new(2.0) / F::new(9.0) * t16255 - F::new(4.0) / F::new(9.0) * t11846 - F::new(2.0) / F::new(3.0) * t16296 + F::new(2.0) / F::new(27.0) * t16298 + t16300 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t16302 - F::new(4.0) / F::new(27.0) * t11883 - t8534 - F::new(2.0) / F::new(3.0) * t16490 - F::new(2.0) / F::new(3.0) * t16539;
    (t20292, t20304)
}
