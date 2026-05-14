//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 154/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk154<F: Float>(t526: F, t558: F, t27: F, t89: F, t518: F, t522: F, t515: F, t161: F, t376: F, t143: F, t378: F) -> (F, F, F, F, F) {
    let t559 = t526 * t558;
    let t561 = t89 * t27 * t559;
    let t563 = -t518 - t522 / 18.0 - t561 / 6.0;
    let t564 = t515 * t563;
    let t568 = t89 * t376 * t161 / 9.0;
    let t569 = t378 * t143;
    (t559, t561, t564, t568, t569)
}
