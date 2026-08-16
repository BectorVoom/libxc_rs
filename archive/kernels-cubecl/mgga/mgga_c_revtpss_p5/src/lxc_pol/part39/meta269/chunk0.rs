//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1000/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1000<F: Float>(t1333: F, t3860: F, t4144: F, t4147: F, t30: F, t513: F, t33: F, t516: F, t2435: F, t3900: F, t212: F, t4066: F) -> (F, F, F, F, F, F) {
    let t9597 = t3860 * t1333;
    let t9599 = t4144 * t4147;
    let t9603 = t30 * t30;
    let t9605 = F::cast_from(1.0_f64) / t513 / t9603;
    let t9615 = t33 * t33;
    let t9617 = F::cast_from(1.0_f64) / t516 / t9615;
    let t9632 = t2435 * t3900;
    let t9634 = t212 * t4066;
    (t9597, t9599, t9605, t9617, t9632, t9634)
}
