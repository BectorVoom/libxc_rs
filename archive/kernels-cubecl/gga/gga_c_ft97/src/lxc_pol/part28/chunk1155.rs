//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1155/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1155<F: Float>(t34853: F, t446: F, t558: F, t9432: F, t1369: F, t34919: F, t376: F, t148249: F, t2112: F, t28: F, t148412: F, t9073: F) -> (F, F, F, F) {
    let t148657 = t446 * t9432 * t34853 * t558;
    let t148660 = t1369 * t376 * t34919;
    let t148667 = t1369 * t28 * t2112 * t148249;
    let t148670 = t446 * t9073 * t148412;
    (t148657, t148660, t148667, t148670)
}
