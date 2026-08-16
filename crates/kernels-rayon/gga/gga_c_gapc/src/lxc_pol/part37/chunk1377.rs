//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1377/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1377(t33760: f64, t33763: f64, t33770: f64, t33772: f64, t33774: f64, t33777: f64, t33784: f64, t33787: f64, t33789: f64, t33791: f64, t33793: f64, t33796: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36694 = 0.13259130899812740005e-6_f64 * t33760;
    let t36695 = 0.44197102999375800018e-8_f64 * t33763;
    let t36698 = 0.10567613244746075633e-6_f64 * t33770;
    let t36699 = 0.40021712703254065176e-7_f64 * t33772;
    let t36700 = 0.40094868252346065012e-6_f64 * t33774;
    let t36701 = 0.66295654499063700026e-7_f64 * t33777;
    let t36703 = 0.19336232562226912508e-8_f64 * t33784;
    let t36704 = 0.2845640240200497334e-7_f64 * t33787;
    let t36705 = 0.34782544165564226085e-4_f64 * t33789;
    let t36706 = 0.42205124476153752644e-7_f64 * t33791;
    let t36707 = 0.33764099580923002116e-6_f64 * t33793;
    let t36708 = 0.21102562238076876322e-7_f64 * t33796;
    (t36694, t36695, t36698, t36699, t36700, t36701, t36703, t36704, t36705, t36706, t36707, t36708)
}
