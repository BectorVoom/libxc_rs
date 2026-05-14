//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1343/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1343<F: Float>(t10248: F, t126368: F, t446: F, t1486: F, t31628: F, t681: F, t24980: F, t24981: F, t28533: F, t28729: F, t126447: F, t6317: F, t113169: F, t113177: F, t113196: F, t113202: F, t113227: F, t113244: F, t113250: F, t99737: F) -> (F, F, F, F, F) {
    let t126814 = t446 * t10248 * t126368;
    let t126817 = t1486 * t681 * t31628;
    let t126818 = t126817 / 3.0;
    let t126821 = t24980 * t24981 * t28533 * t28729;
    let t126824 = t6317 * t24981 * t126447;
    let t126826 = -2.0 / 3.0 * t126814 - t126818 - t113169 - t113177 - t126821 / 6.0 + t113196 + t113202 + t113227 + t99737 - 2.0 / 3.0 * t126824 + t113244 + t113250;
    (t126814, t126817, t126821, t126824, t126826)
}
