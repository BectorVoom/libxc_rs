//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 760/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk760<F: Float>(t2683: F, t375: F, t89: F, t793: F, t9733: F, t2336: F, t2675: F, t10243: F, t10246: F, t10251: F, t10255: F, t10259: F, t10265: F, t10269: F, t10273: F) -> (F, F, F, F) {
    let t10276 = t89 * t375 * t2683;
    let t10279 = t89 * t9733 * t793;
    let t10282 = t89 * t2336 * t2675;
    let t10284 = -t10243 / F::new(9.0) - t10246 / F::new(9.0) - t10251 / F::new(3.0) - t10255 / F::new(3.0) - t10259 / F::new(18.0) - t10265 + t10269 - F::new(5.0) / F::new(81.0) * t10273 - t10276 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t10279 + t10282 / F::new(18.0);
    (t10276, t10279, t10282, t10284)
}
