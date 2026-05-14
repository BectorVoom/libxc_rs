//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1261/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1261<F: Float>(t30155: F, t92: F, t17355: F, t5935: F, t105679: F, t23657: F, t3450: F, t6656: F, t16169: F, t23909: F, t27072: F, t27142: F, t4417: F, t590: F, t16988: F, t5916: F) -> (F, F, F, F, F, F, F, F) {
    let t119546 = t30155 * t92;
    let t119550 = t5935 * t17355;
    let t119556 = t23657 * t105679 * t6656 * t3450;
    let t119558 = t23909 * t16169;
    let t119560 = t27142 * t27072 * t119558;
    let t119562 = t4417 * t590;
    let t119565 = t23657 * t27072 * t23909 * t119562;
    let t119567 = t5916 * t16988;
    (t119546, t119550, t119556, t119558, t119560, t119562, t119565, t119567)
}
