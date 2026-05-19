//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1379/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1379<F: Float>(t33760: F, t33763: F, t33770: F, t33772: F, t33774: F, t33777: F, t33784: F, t33787: F, t33789: F, t33791: F, t33793: F, t33796: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36694 = F::cast_from(0.13259130899812740005e-6_f64) * t33760;
    let t36695 = F::cast_from(0.44197102999375800018e-8_f64) * t33763;
    let t36698 = F::cast_from(0.10567613244746075633e-6_f64) * t33770;
    let t36699 = F::cast_from(0.40021712703254065176e-7_f64) * t33772;
    let t36700 = F::cast_from(0.40094868252346065012e-6_f64) * t33774;
    let t36701 = F::cast_from(0.66295654499063700026e-7_f64) * t33777;
    let t36703 = F::cast_from(0.19336232562226912508e-8_f64) * t33784;
    let t36704 = F::cast_from(0.2845640240200497334e-7_f64) * t33787;
    let t36705 = F::cast_from(0.34782544165564226085e-4_f64) * t33789;
    let t36706 = F::cast_from(0.42205124476153752644e-7_f64) * t33791;
    let t36707 = F::cast_from(0.33764099580923002116e-6_f64) * t33793;
    let t36708 = F::cast_from(0.21102562238076876322e-7_f64) * t33796;
    (t36694, t36695, t36698, t36699, t36700, t36701, t36703, t36704, t36705, t36706, t36707, t36708)
}
