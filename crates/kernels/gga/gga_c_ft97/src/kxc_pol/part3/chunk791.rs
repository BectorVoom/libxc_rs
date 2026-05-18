//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 791/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk791<F: Float>(t16279: F, t83: F, t11846: F, t11849: F, t16230: F, t16234: F, t16238: F, t16243: F, t16248: F, t16252: F, t16255: F, t16258: F, t16263: F, t16268: F, t16272: F, t16276: F, t1901: F, t446: F) -> F {
    let t16280 = t83 * t16279;
    let t16284 = -F::new(2.0) / F::new(9.0) * t1901 * t16230 + F::new(2.0) / F::new(27.0) * t1901 * t16234 + F::new(2.0) / F::new(27.0) * t1901 * t16238 + t1901 * t16243 / F::new(9.0) - t446 * t16248 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t16252 - F::new(2.0) / F::new(27.0) * t16255 + F::new(2.0) / F::new(3.0) * t446 * t16258 + F::new(2.0) / F::new(3.0) * t446 * t16263 + t446 * t16268 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t16272 + F::new(2.0) / F::new(3.0) * t446 * t16276 + F::new(4.0) / F::new(3.0) * t446 * t16280 - F::new(8.0) / F::new(27.0) * t11846 + t11849;
    t16284
}
