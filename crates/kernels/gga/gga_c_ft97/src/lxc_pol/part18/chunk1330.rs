//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1330/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1330<F: Float>(t26909: F, t446: F, t558: F, t9432: F, t105357: F, t39749: F, t1017: F, t23884: F, t1369: F, t2112: F, t28: F, t105629: F, t105633: F, t105638: F, t105641: F, t105645: F, t105649: F, t105653: F, t95177: F, t96086: F) -> (F, F, F, F, F) {
    let t105657 = t446 * t9432 * t26909 * t558;
    let t105660 = t446 * t39749 * t105357;
    let t105662 = t23884 * t1017;
    let t105665 = t1369 * t28 * t2112 * t105662;
    let t105666 = t105629 / 2.0 - t105633 / 6.0 + 16.0 / 9.0 * t95177 - t105638 + 2.0 * t105641 + 2.0 * t105645 - 6.0 * t105649 - t105653 / 18.0 + t96086 - 12.0 * t105657 + 2.0 * t105660 + t105665;
    (t105657, t105660, t105662, t105665, t105666)
}
