//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1119/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1119(t10529: f64, t14587: f64, t2782: f64, t4469: f64, t72: f64, t686: f64, t874: f64, t1558: f64, t2811: f64, t2482: f64, t122: f64, t2723: f64) -> (f64, f64, f64, f64) {
    let t14588 = t10529 * t14587;
    let t14590 = 0.21951497276451705328e-1_f64 * t2782 * t14588;
    let t14593 = t4469 * t72;
    let t14596 = 0.19514881078765566038e-1_f64 * t874 * t14593 * t686;
    let t14597 = t2811 * t1558;
    let t14598 = t2482 * t14597;
    let t14600 = t2723 * t72 * t122;
    (t14590, t14596, t14598, t14600)
}
