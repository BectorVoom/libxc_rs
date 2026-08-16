//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1349/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1349(t11607: f64, t1186: f64, t11925: f64, t1238: f64, t2154: f64, t24633: f64, t24638: f64, t24877: f64, t24883: f64, t27799: f64, t3471: f64, t3477: f64, t3593: f64, t45350: f64, t7283: f64, t7300: f64, t7302: f64, t7392: f64, t85674: f64, t85683: f64, t85688: f64, t85701: f64, t85707: f64, t85711: f64) -> f64 {
    let t85713 = -0.49348022005446793095e-1_f64 * t7283 * t7300 * t85674 * t11607 + 24.0_f64 * t1238 * t45350 * t2154 * t11607 - 0.24674011002723396548e-1_f64 * t7283 * t85683 * t27799 - 0.49348022005446793095e-1_f64 * t7283 * t1186 * t85688 + 0.24674011002723396548e-1_f64 * t7283 * t3471 * t24638 + 0.24674011002723396548e-1_f64 * t7283 * t3477 * t24638 - 0.82246703342411321826e-2_f64 * t7283 * t24633 * t24883 + 0.54831135561607547884e-2_f64 * t85701 + 6.0_f64 * t3593 * t24877 - 3.0_f64 * t11925 * t7392 - 0.24674011002723396548e-1_f64 * t7283 * t85707 * t7302 - 0.82246703342411321826e-2_f64 * t85711;
    t85713
}
