//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1237/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1237<F: Float>(t221: F, t4433: F, t10703: F, t2674: F, t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t1544: F, t2430: F, t2477: F, t828: F) -> (F, F, F, F) {
    let t14756 = t221 * t4433;
    let t14757 = t10703 * t14756;
    let t14759 = F::cast_from(0.50820002809285328225e-3_f64) * t2674 * t14757;
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    let t14767 = t1544 * t2430;
    let t14769 = t2477 * t828 * t14767;
    (t14759, t14761, t14765, t14769)
}
