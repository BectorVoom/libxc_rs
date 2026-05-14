//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 990/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk990<F: Float>(t28: F, t3526: F, t586: F, t5890: F, t7339: F, t32924: F, t3450: F, t40830: F, t5899: F, t26909: F, t5900: F, t9432: F, t23649: F, t34850: F, t139241: F, t139254: F, t139257: F, t139275: F, t139278: F, t148334: F, t148338: F, t148342: F, t148346: F, t148349: F, t148353: F, t148360: F) -> (F, F, F, F, F) {
    let t148365 = t5890 * t28 * t586 * t7339 * t3526;
    let t148369 = t5899 * t40830 * t32924 * t3450;
    let t148373 = t5899 * t9432 * t5900 * t26909;
    let t148375 = t23649 * t34850;
    let t148377 = -4.0 / 27.0 * t148334 - t148338 / 9.0 - 4.0 / 27.0 * t139241 - t148342 / 9.0 - t148346 / 36.0 + t148349 / 18.0 - t148353 / 3.0 + 4.0 / 9.0 * t139254 - t139257 / 3.0 - t139275 / 36.0 + t139278 / 18.0 + 4.0 / 9.0 * t148360 + t148365 / 12.0 + 4.0 * t148369 - 2.0 * t148373 - t148375 / 54.0;
    (t148365, t148369, t148373, t148375, t148377)
}
