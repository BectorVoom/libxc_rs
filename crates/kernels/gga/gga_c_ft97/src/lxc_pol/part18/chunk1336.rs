//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1336/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1336<F: Float>(t105743: F, t105448: F, t446: F, t9049: F, t2087: F, t23609: F, t28: F, t586: F, t6615: F, t105518: F, t9073: F, t105444: F, t1969: F, t1369: F, t27131: F, t376: F) -> (F, F, F, F, F, F) {
    let t105744 = 2.0 / 3.0 * t105743;
    let t105746 = t446 * t9049 * t105448;
    let t105751 = t23609 * t28 * t586 * t6615 * t2087;
    let t105754 = t446 * t9073 * t105518;
    let t105757 = t446 * t1969 * t105444;
    let t105760 = t1369 * t376 * t27131;
    (t105744, t105746, t105751, t105754, t105757, t105760)
}
