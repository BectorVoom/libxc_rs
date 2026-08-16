//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2526/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2526<F: Float>(t51564: F, t10115: F, t1576: F, t14593: F, t2470: F, t874: F, t10538: F, t14605: F, t49180: F, t10535: F, t136: F, t2457: F, t4424: F) -> (F, F, F, F, F) {
    let t51565 = F::cast_from(0.34697458558045176417e-2_f64) * t51564;
    let t51578 = t10115 * t1576;
    let t51587 = t874 * t14593 * t2470;
    let t51588 = F::cast_from(0.39029762157531132076e-1_f64) * t51587;
    let t51603 = t49180 * t14605 * t10538;
    let t51604 = F::cast_from(0.34697458558045176417e-2_f64) * t51603;
    let t51614 = t10535 * t4424 * t136 * t2457;
    (t51565, t51578, t51588, t51604, t51614)
}
