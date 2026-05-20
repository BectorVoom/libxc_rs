//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1165/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1165<F: Float>(t1749: F, t3520: F, t16868: F, t16712: F, t16892: F, t16708: F, t3495: F, t1770: F, t3781: F, t1284: F, t1811: F, t1209: F) -> (F, F, F, F, F, F, F, F) {
    let t17097 = t1749 * t3520;
    let t17115 = F::new(0.11038e0) * t16868;
    let t17117 = F::cast_from(0.20128333333333333334e0_f64) * t16712;
    let t17131 = F::new(0.22076e0) * t16892;
    let t17140 = F::cast_from(0.13418888888888888889e0_f64) * t16708;
    let t17154 = t1749 * t3495;
    let t17183 = t1770 * t3781;
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    (t17097, t17115, t17117, t17131, t17140, t17154, t17183, t17192)
}
