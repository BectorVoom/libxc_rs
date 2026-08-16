//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1499/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1499(t14593: f64, t686: f64, t874: f64, t1558: f64, t2811: f64, t2482: f64, t122: f64, t2723: f64, t72: f64, t676: f64, t836: f64, t879: f64) -> (f64, f64, f64) {
    let t14596 = 0.19514881078765566038e-1_f64 * t874 * t14593 * t686;
    let t14597 = t2811 * t1558;
    let t14598 = t2482 * t14597;
    let t14600 = t2723 * t72 * t122;
    let t14602 = t14600 * t676 * t836;
    let t14603 = t14598 * t14602;
    let t14605 = t879 * t1558;
    (t14596, t14603, t14605)
}
