//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1122/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1122<F: Float>(t1613: F, t6817: F, t92354: F, t2387: F, t27660: F, t420: F, t200: F, t668: F, t27521: F, t27523: F, t27574: F, t172: F, t231: F, t27616: F, t27620: F, t3794: F, t70: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108524 = t92354 * t1613 * t6817;
    let t108525 = t2387 * t108524;
    let t108526 = t420 * t27660;
    let t108530 = t200 * t668;
    let t108550 = 0.60548059007656442388e-3 * t27521 * t27574 * t27523;
    let t108572 = t6817 * t172;
    let t108573 = t108572 * t231;
    let t108576 = 0.3520097786805302698e-5 * t27616 * t108573 * t27620;
    let t108581 = t3794 * t70;
    (t108524, t108525, t108526, t108530, t108550, t108572, t108573, t108576, t108581)
}
