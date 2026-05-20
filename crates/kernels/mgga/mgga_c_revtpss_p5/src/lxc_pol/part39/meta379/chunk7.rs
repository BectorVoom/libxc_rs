//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1362/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1362<F: Float>(t1149: F, t5105: F, t3384: F, t1733: F, t3427: F, t3385: F, t5108: F, t12248: F, t3435: F, t5104: F, t3433: F, t12230: F, t1732: F) -> (F, F, F, F, F, F) {
    let t16652 = t5105 * t1149;
    let t16654 = F::new(4.0) * t3384 * t16652;
    let t16655 = t1733 * t3427;
    let t16657 = F::new(2.0) * t3384 * t16655;
    let t16658 = t5108 * t3385;
    let t16660 = F::cast_from(0.96491876992155210402e2_f64) * t12248 * t16658;
    let t16661 = t5104 * t3435;
    let t16662 = t16661 * t1149;
    let t16664 = F::cast_from(0.32163958997385070134e2_f64) * t3433 * t16662;
    let t16665 = t5108 * t3427;
    let t16667 = F::cast_from(0.16081979498692535067e2_f64) * t3433 * t16665;
    let t16668 = t1732 * t12230;
    (t16654, t16657, t16660, t16664, t16667, t16668)
}
