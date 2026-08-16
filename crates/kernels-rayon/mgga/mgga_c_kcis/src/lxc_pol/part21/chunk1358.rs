//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1358/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1358(t1267: f64, t26975: f64, t5329: f64, t5341: f64, t11081: f64, t26960: f64, t28106: f64, t1856: f64, t3616: f64, t7772: f64, t96727: f64, t1851: f64, t26996: f64) -> (f64, f64, f64, f64, f64) {
    let t97039 = t5329 * t26975 * t5341 * t1267;
    let t97051 = 0.7722800925925925926e-4_f64 * t26960 * t11081 * t28106;
    let t97056 = t5329 * t26975 * t1856 * t3616;
    let t97060 = 0.92754700520833333333e-4_f64 * t7772 * t96727;
    let t97063 = t5329 * t26996 * t1851 * t3616;
    (t97039, t97051, t97056, t97060, t97063)
}
