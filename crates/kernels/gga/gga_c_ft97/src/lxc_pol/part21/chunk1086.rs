//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1086/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1086<F: Float>(t6584: F, t94983: F, t458: F, t6579: F, t5775: F, t23405: F, t27423: F, t27429: F, t11176: F, t1348: F, t26811: F, t1349: F, t26552: F, t376: F, t1637: F, t6617: F) -> (F, F, F, F, F, F, F, F) {
    let t104541 = t94983 * t6584;
    let t104547 = t6579 * t458;
    let t104549 = t104547 * t5775 / 27.0;
    let t104552 = 2.0 / 27.0 * t23405 * t27423;
    let t104554 = 2.0 / 81.0 * t23405 * t27429;
    let t104562 = t1348 * t11176 * t26811;
    let t104599 = 2.0 / 9.0 * t1349 * t376 * t26552;
    let t104619 = t1349 * t1637 * t6617;
    (t104541, t104547, t104549, t104552, t104554, t104562, t104599, t104619)
}
