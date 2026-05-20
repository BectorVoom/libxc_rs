//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2587/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2587<F: Float>(t20567: F, t448: F, t17092: F, t5068: F, t16840: F, t5109: F, t1149: F, t6439: F, t3433: F, t1733: F, t5104: F, t3384: F) -> (F, F, F, F, F, F, F) {
    let t20568 = t20567 * t448;
    let t20571 = F::new(4.0) * t17092 * t5068;
    let t20573 = F::cast_from(0.32163958997385070134e2_f64) * t16840 * t5109;
    let t20574 = t6439 * t1149;
    let t20576 = F::new(6.0) * t3433 * t20574;
    let t20577 = t1733 * t5104;
    let t20579 = F::new(4.0) * t3384 * t20577;
    (t20568, t20571, t20573, t20574, t20576, t20577, t20579)
}
