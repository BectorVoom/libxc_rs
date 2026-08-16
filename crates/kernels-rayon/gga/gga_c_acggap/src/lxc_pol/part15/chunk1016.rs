//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1016/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1016(t31505: f64, t31530: f64, t31532: f64, t1967: f64, t8502: f64, t1998: f64, t5089: f64, t1451: f64, t7605: f64, t1423: f64, t7736: f64, t30318: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35713 = 0.18007087609589289529e-1_f64 * t31505;
    let t35718 = 0.34299214494455789578e-2_f64 * t31530;
    let t35719 = 0.34299214494455789578e-2_f64 * t31532;
    let t35722 = t1967 * t8502;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    (t35713, t35718, t35719, t35722, t35733, t35736, t35738, t35740)
}
