//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1377/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1377<F: Float>(t33760: F, t33763: F, t33770: F, t33772: F, t33774: F, t33777: F, t33784: F, t33787: F, t33789: F, t33791: F, t33793: F, t33796: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36694 = F::new(0.13259130899812740005e-6) * t33760;
    let t36695 = F::new(0.44197102999375800018e-8) * t33763;
    let t36698 = F::new(0.10567613244746075633e-6) * t33770;
    let t36699 = F::new(0.40021712703254065176e-7) * t33772;
    let t36700 = F::new(0.40094868252346065012e-6) * t33774;
    let t36701 = F::new(0.66295654499063700026e-7) * t33777;
    let t36703 = F::new(0.19336232562226912508e-8) * t33784;
    let t36704 = F::new(0.2845640240200497334e-7) * t33787;
    let t36705 = F::new(0.34782544165564226085e-4) * t33789;
    let t36706 = F::new(0.42205124476153752644e-7) * t33791;
    let t36707 = F::new(0.33764099580923002116e-6) * t33793;
    let t36708 = F::new(0.21102562238076876322e-7) * t33796;
    (t36694, t36695, t36698, t36699, t36700, t36701, t36703, t36704, t36705, t36706, t36707, t36708)
}
