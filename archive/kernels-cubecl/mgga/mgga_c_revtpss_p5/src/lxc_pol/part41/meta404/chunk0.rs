//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1399/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1399<F: Float>(t473: F, t6695: F, t1214: F, t3759: F, t6587: F, t1280: F, t21082: F, t21471: F, t5284: F, t5332: F, t1269: F, t1287: F, t6622: F) -> (F, F, F, F, F) {
    let t21541 = t473 * t6695;
    let t21542 = t21541 * t1214;
    let t21551 = t3759 * t6587;
    let t21554 = t1280 * t21082;
    let t21557 = t21471 * t5284;
    let t21558 = t5332 * t21557;
    let t21562 = t1269 * t6622 * t1287;
    (t21542, t21551, t21554, t21558, t21562)
}
