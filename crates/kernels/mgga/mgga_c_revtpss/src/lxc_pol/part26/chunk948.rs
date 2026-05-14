//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 948/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk948<F: Float>(t114: F, t26028: F, t3940: F, t3926: F, t7264: F, t25304: F, t7283: F, t25949: F, t786: F, t1426: F, t3999: F, t25821: F, t25824: F, t25827: F, t25829: F) -> (F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t26029 = t26028 * t3940;
    let t26031 = t7264 * t3926;
    let t26069 = t25304 * t7283;
    let t26072 = t786 * t25949;
    let t26079 = t1426 * t3999;
    let t26148 = 22.0 / 9.0 * t25821;
    let t26153 = piecewise3(t115, 0.0, t26148 + 4.0 / 3.0 * t25824 + t25827 / 2.0 - t25829 / 4.0);
    (t26029, t26031, t26069, t26072, t26079, t26153)
}
