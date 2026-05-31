//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 849/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk849<F: Float>(t45108: F, t13548: F, t731: F, t11832: F, t22090: F, t2508: F, t7291: F, t11969: F, t2592: F, t10301: F, t8045: F, t11714: F, t7324: F) -> (F, F, F, F, F, F) {
    let t45109 = F::cast_from(0.32043859292259267849e-3_f64) * t45108;
    let t45110 = t731 * t13548;
    let t45111 = F::cast_from(0.42725145723012357132e-3_f64) * t45110;
    let t45115 = F::cast_from(0.1845726295234133828e0_f64) * t2508 * t22090 * t11832 * t7291;
    let t45124 = t2592 * t11969;
    let t45134 = F::cast_from(4.0_f64) * t8045 * t10301;
    let t45141 = F::cast_from(4.0_f64) * t7324 * t11714;
    (t45109, t45111, t45115, t45124, t45134, t45141)
}
