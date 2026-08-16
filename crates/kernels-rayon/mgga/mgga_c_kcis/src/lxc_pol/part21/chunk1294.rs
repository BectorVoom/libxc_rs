//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1294/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1294(t27789: f64, t2861: f64, t27793: f64, t1092: f64, t27788: f64, t92917: f64, t27842: f64, t2822: f64, t13398: f64, t27846: f64, t14430: f64, t2811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95815 = t2861 * t27789;
    let t95816 = 0.22109259259259259258e-2_f64 * t95815;
    let t95817 = t2861 * t27793;
    let t95820 = t1092 * t92917 * t27788;
    let t95826 = t2822 * t27842;
    let t95827 = 0.22109259259259259258e-2_f64 * t95826;
    let t95828 = t13398 * t27846;
    let t95830 = t14430 * t2811;
    (t95815, t95816, t95817, t95820, t95826, t95827, t95828, t95830)
}
