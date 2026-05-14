//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1270/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1270<F: Float>(t113249: F, t113251: F, t113268: F, t113248: F, t113253: F, t113257: F, t113261: F, t113265: F, t113270: F, t113273: F, t113277: F, t113282: F, t113295: F, t113325: F, t113289: F, t113293: F, t113298: F, t113301: F, t113304: F, t113307: F, t113311: F, t113314: F, t113318: F, t113322: F) -> (F, F) {
    let t114340 = 4.0 / 3.0 * t113249;
    let t114341 = 2.0 / 27.0 * t113251;
    let t114346 = t113268 / 9.0;
    let t114351 = -t113248 / 3.0 + t114340 - t114341 + 4.0 / 81.0 * t113253 - 4.0 / 9.0 * t113257 - 4.0 * t113261 - t113265 / 6.0 + t114346 + 22.0 / 27.0 * t113270 - 2.0 / 27.0 * t113273 + t113277 / 9.0 + 2.0 / 27.0 * t113282;
    let t114355 = 2.0 / 27.0 * t113295;
    let t114364 = t113325 / 18.0;
    let t114365 = -2.0 / 9.0 * t113289 + 8.0 * t113293 - t114355 + 11.0 / 27.0 * t113298 - 2.0 / 9.0 * t113301 - 4.0 / 9.0 * t113304 + 8.0 / 9.0 * t113307 + 4.0 / 27.0 * t113311 - 2.0 / 9.0 * t113314 + 8.0 / 9.0 * t113318 - 4.0 / 9.0 * t113322 - t114364;
    (t114351, t114365)
}
