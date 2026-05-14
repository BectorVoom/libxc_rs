//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 683/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk683<F: Float>(t33476: F, t505: F, t2354: F, t446: F, t33253: F, t713: F, t193: F, t89: F, t33452: F, t676: F, t27: F, t33340: F, t33344: F, t33349: F, t33455: F, t33459: F, t33463: F, t33467: F, t33471: F, t33475: F) -> (F, F, F, F, F, F, F) {
    let t33477 = t33476 * t505;
    let t33478 = t2354 * t33477;
    let t33479 = t446 * t33478;
    let t33481 = t33253 * t713;
    let t33482 = t193 * t33481;
    let t33483 = t89 * t33482;
    let t33485 = t676 * t33452;
    let t33487 = t89 * t27 * t33485;
    let t33488 = t33340 + t33344 / 6.0 + t33349 - t33455 / 2.0 - t33459 - 2.0 / 3.0 * t33463 - 6.0 * t33467 + 4.0 * t33471 + t33475 + t33479 / 3.0 + 2.0 * t33483 - t33487;
    (t33478, t33479, t33481, t33483, t33485, t33487, t33488)
}
