//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1273/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1273<F: Float>(t23405: F, t26785: F, t24094: F, t6580: F, t26581: F, t5769: F, t6615: F, t7368: F, t1349: F, t23401: F, t24127: F, t24131: F, t24148: F, t28: F, t5849: F, t6622: F, t94191: F, t94198: F, t94201: F, t94206: F, t94214: F) -> (F, F) {
    let t104217 = t23405 * t26785 / 27.0;
    let t104220 = t6580 * t24094 / 9.0;
    let t104225 = t26581 * t5769 / 9.0;
    let t104235 = t7368 * t6615;
    let t104239 = t104217 + 4.0 / 27.0 * t94191 - t104220 + 4.0 / 27.0 * t94198 + t24148 * t6622 / 6.0 - t104225 + t6580 * t24127 / 6.0 - 8.0 / 27.0 * t94201 + 2.0 / 9.0 * t94206 - t94214 / 3.0 + t6580 * t24131 / 6.0 + t26581 * t5849 / 3.0 + t1349 * t28 * t104235 * t23401;
    (t104235, t104239)
}
