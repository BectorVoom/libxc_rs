//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2188/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2188<F: Float>(t24987: F, t7688: F, t1874: F, t75560: F, t19451: F, t6525: F, t25994: F, t4028: F, t55943: F, t191: F, t192: F, t19537: F) -> (F, F, F, F, F, F) {
    let t97794 = F::cast_from(6.0_f64) * t24987 * t7688;
    let t97796 = F::cast_from(2.0_f64) * t75560 * t1874;
    let t97798 = F::cast_from(2.0_f64) * t19451 * t6525;
    let t97800 = F::cast_from(4.0_f64) * t4028 * t25994;
    let t97802 = F::cast_from(2.0_f64) * t55943 * t1874;
    let t97804 = t19537 * t191 * t192;
    (t97794, t97796, t97798, t97800, t97802, t97804)
}
