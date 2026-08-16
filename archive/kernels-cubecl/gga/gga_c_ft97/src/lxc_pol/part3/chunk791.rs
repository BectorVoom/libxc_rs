//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 791/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk791<F: Float>(t16279: F, t83: F, t11846: F, t11849: F, t16230: F, t16234: F, t16238: F, t16243: F, t16248: F, t16252: F, t16255: F, t16258: F, t16263: F, t16268: F, t16272: F, t16276: F, t1901: F, t446: F) -> F {
    let t16280 = t83 * t16279;
    let t16284 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16230 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t16234 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t16238 + t1901 * t16243 / F::cast_from(9.0_f64) - t446 * t16248 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16252 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16255 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16258 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16263 + t446 * t16268 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t16272 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16276 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t16280 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11846 + t11849;
    t16284
}
