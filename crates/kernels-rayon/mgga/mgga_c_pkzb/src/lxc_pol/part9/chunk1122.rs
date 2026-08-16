//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1122/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1122(t19191: f64, t2380: f64, t2383: f64, t6475: f64, t6484: f64, t53: f64, t6404: f64, t179: f64, t404: f64, t6406: f64, t414: f64, t6545: f64) -> (f64, f64, f64, f64) {
    let t19193 = t2380 * t19191 * t2383;
    let t19196 = t2380 * t6475 * t6484;
    let t19203 = t53 * t6404;
    let t19206 = t404 * t179 * t19203 * t6406;
    let t19227 = 1.0_f64 / t6545 / t414;
    (t19193, t19196, t19206, t19227)
}
