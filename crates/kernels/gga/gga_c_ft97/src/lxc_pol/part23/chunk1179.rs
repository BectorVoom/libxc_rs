//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1179/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1179<F: Float>(t113243: F, t113249: F, t113251: F, t113268: F, t113295: F, t113325: F, t113329: F, t113332: F, t113339: F, t113346: F, t113348: F, t113356: F, t113372: F, t113446: F, t113465: F, t113564: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t114337 = t113243 / 9.0;
    let t114340 = 4.0 / 3.0 * t113249;
    let t114341 = 2.0 / 27.0 * t113251;
    let t114346 = t113268 / 9.0;
    let t114355 = 2.0 / 27.0 * t113295;
    let t114364 = t113325 / 18.0;
    let t114366 = t113329 / 18.0;
    let t114367 = 2.0 / 9.0 * t113332;
    let t114370 = 4.0 / 81.0 * t113339;
    let t114372 = t113346 / 18.0;
    let t114373 = t113348 / 27.0;
    let t114375 = t113356 / 27.0;
    let t114384 = 4.0 / 27.0 * t113372;
    let t114415 = 2.0 / 9.0 * t113446;
    let t114420 = t113465 / 12.0;
    let t114452 = 2.0 / 27.0 * t113564;
    (t114337, t114340, t114341, t114346, t114355, t114364, t114366, t114367, t114370, t114372, t114373, t114375, t114384, t114415, t114420, t114452)
}
