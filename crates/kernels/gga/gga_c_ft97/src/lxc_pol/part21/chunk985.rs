//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 985/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk985<F: Float>(t1901: F, t23950: F, t27278: F, t27302: F, t28: F, t30447: F, t30451: F, t30454: F, t30458: F, t30461: F, t30465: F, t30469: F, t30474: F, t30479: F, t30483: F, t30486: F, t30491: F, t30496: F, t30500: F, t446: F, t89: F) -> (F,) {
    let t30505 = -2.0 / 9.0 * t27278 - 2.0 / 9.0 * t1901 * t30447 - 4.0 / 9.0 * t1901 * t30451 - 4.0 / 9.0 * t1901 * t30454 - 2.0 / 9.0 * t1901 * t30458 + 4.0 / 3.0 * t446 * t30461 - t446 * t30465 / 3.0 + 2.0 / 3.0 * t446 * t30469 + 2.0 / 3.0 * t446 * t30474 - 2.0 / 3.0 * t446 * t30479 - 2.0 * t446 * t30483 - 2.0 / 3.0 * t446 * t30486 + 2.0 / 3.0 * t446 * t30491 + t446 * t30496 / 3.0 + t89 * t28 * t30500 / 3.0 - 2.0 / 9.0 * t27302 - t23950;
    (t30505,)
}
