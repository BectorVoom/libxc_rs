//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1187/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1187(t7703: f64, t95684: f64, t14443: f64, t27957: f64, t27953: f64, t9938: f64, t27789: f64, t2861: f64, t27793: f64, t27842: f64, t2822: f64, t13398: f64, t27846: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95779 = 0.46336805555555555556e-3_f64 * t7703 * t95684;
    let t95781 = t14443 * t27957;
    let t95783 = 0.15445601851851851852e-3_f64 * t7703 * t95781;
    let t95798 = 0.15445601851851851852e-3_f64 * t7703 * t9938 * t27953;
    let t95815 = t2861 * t27789;
    let t95816 = 0.22109259259259259258e-2_f64 * t95815;
    let t95817 = t2861 * t27793;
    let t95826 = t2822 * t27842;
    let t95827 = 0.22109259259259259258e-2_f64 * t95826;
    let t95828 = t13398 * t27846;
    (t95779, t95781, t95783, t95798, t95815, t95816, t95817, t95826, t95827, t95828)
}
