//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 845/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk845(t1445: f64, t5789: f64, t532: f64, t5793: f64, t1409: f64, t167: f64, t5801: f64, t1401: f64, t5805: f64, t4023: f64, t1441: f64, t1650: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17045 = 0.93706135855523581992e-2_f64 * t1445 * t5789;
    let t17047 = 0.93706135855523581992e-2_f64 * t532 * t5793;
    let t17057 = t1409 * t167;
    let t17062 = 0.93706135855523581992e-2_f64 * t532 * t5801;
    let t17065 = 0.28111840756657074598e-1_f64 * t1401 * t5805;
    let t17088 = t4023 * t1409;
    let t17096 = t1441 * t1650;
    (t17045, t17047, t17057, t17062, t17065, t17088, t17096)
}
