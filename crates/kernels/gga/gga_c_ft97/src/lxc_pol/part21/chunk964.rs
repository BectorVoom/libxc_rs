//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 964/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk964<F: Float>(t28: F, t30161: F, t1349: F, t1362: F, t26556: F, t26565: F, t26572: F, t26817: F, t30034: F, t30108: F, t30112: F, t30119: F, t30124: F, t30128: F, t30131: F, t30134: F, t30137: F, t30141: F, t30145: F, t30149: F, t30156: F, t5772: F, t6580: F, t6584: F, t6589: F) -> (F, F) {
    let t30162 = t28 * t30161;
    let t30166 = t1349 * t30034 / 3.0 + t1349 * t30108 / 6.0 - 2.0 / 3.0 * t1349 * t30112 - 2.0 / 3.0 * t6580 * t6589 - 2.0 / 3.0 * t1349 * t30119 - t1349 * t30124 / 3.0 - 12.0 * t30128 + 8.0 * t30131 + 4.0 * t30134 - t5772 * t30137 / 9.0 - t5772 * t30141 / 9.0 - t5772 * t30145 / 18.0 - t5772 * t30149 / 27.0 - t26817 * t6584 / 9.0 + t30156 * t1362 / 6.0 - t26556 / 9.0 - t26565 / 9.0 + t1349 * t30162 / 6.0 + t26572 / 27.0;
    (t30162, t30166)
}
