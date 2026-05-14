//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 201/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk201<F: Float>(t198: F, t562: F, t186: F, t561: F, t155: F, t56: F, t174: F, t177: F, t188: F) -> (F, F, F, F, F, F, F) {
    let t563 = t198 * t562;
    let t564 = t186 * t563;
    let t566 = 4.0 / 15.0 * t561 * t564;
    let t567 = t155 * t56;
    let t569 = t174 * t567 * t177;
    let t570 = 0.18891666666666666667e-2 * t569;
    let t571 = t56 * t188;
    (t563, t564, t566, t567, t569, t570, t571)
}
