//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1278/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1278<F: Float>(t119608: F, t446: F, t9073: F, t119612: F, t1969: F, t119596: F, t23900: F, t4822: F, t105544: F, t105560: F, t105568: F, t119828: F, t119832: F, t119837: F, t119842: F, t119847: F) -> (F, F, F, F, F, F) {
    let t119850 = t446 * t9073 * t119608;
    let t119853 = t446 * t1969 * t119612;
    let t119856 = t446 * t9073 * t119596;
    let t119858 = t23900 * t4822;
    let t119860 = t446 * t9073 * t119858;
    let t119862 = -t119828 / 2.0 + t119832 / 3.0 + t119837 / 3.0 + t119842 / 3.0 - t119847 / 9.0 + t105544 - 2.0 / 3.0 * t119850 - 2.0 / 3.0 * t119853 - 2.0 / 3.0 * t119856 - 4.0 / 3.0 * t119860 + t105560 - t105568;
    (t119850, t119853, t119856, t119858, t119860, t119862)
}
