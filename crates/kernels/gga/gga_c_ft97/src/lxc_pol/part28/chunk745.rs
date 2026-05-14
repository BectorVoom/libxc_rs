//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 745/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk745<F: Float>(t1901: F, t33176: F, t33180: F, t33184: F, t33188: F, t33193: F, t33196: F, t33200: F, t33204: F, t33207: F, t33211: F, t33215: F, t33218: F, t446: F, t33049: F, t33130: F, t33174: F) -> (F,) {
    let t33219 = 4.0 / 3.0 * t446 * t33176 + 4.0 / 3.0 * t446 * t33180 - t446 * t33184 / 9.0 + 2.0 / 3.0 * t446 * t33188 - 2.0 / 9.0 * t1901 * t33193 + 2.0 / 9.0 * t1901 * t33196 - 4.0 / 3.0 * t1901 * t33200 - 4.0 / 3.0 * t1901 * t33204 + 2.0 / 3.0 * t446 * t33207 - t446 * t33211 / 3.0 + t33215 - t33218;
    let t33221 = t33049 + t33130 + t33174 + t33219;
    (t33221,)
}
