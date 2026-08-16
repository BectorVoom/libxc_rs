//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2792/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2792(t59022: f64, t12924: f64, t16693: f64, t13127: f64, t16616: f64, t2528: f64, t12908: f64, t16620: f64, t12932: f64, t4205: f64, t47180: f64, t47185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t59023 = 48.0_f64 * t59022;
    let t59024 = t16693 * t12924;
    let t59025 = 48.0_f64 * t59024;
    let t59027 = 48.0_f64 * t16693 * t13127;
    let t59028 = t16616 * t2528;
    let t59029 = 0.17315859105681463759e2_f64 * t59028;
    let t59031 = 24.0_f64 * t12908 * t16620;
    let t59032 = t4205 * t12932;
    let t59033 = 16.0_f64 * t59032;
    let t59034 = 48.0_f64 * t47180;
    let t59035 = 24.0_f64 * t47185;
    (t59023, t59025, t59027, t59029, t59031, t59033, t59034, t59035)
}
