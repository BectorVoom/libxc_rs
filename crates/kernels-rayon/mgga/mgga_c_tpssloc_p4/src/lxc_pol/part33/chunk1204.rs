//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1204/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1204(t1976: f64, t5493: f64, t1458: f64, t7670: f64, t19596: f64, t2019: f64, t1983: f64, t7458: f64, t7468: f64, t1873: f64, t6287: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28852 = t1976 * t5493;
    let t28855 = t7670 * t1458;
    let t28860 = t2019 * t19596;
    let t28861 = t1983 * t28860;
    let t28863 = 4.0_f64 * t7458 * t7468;
    let t28864 = t6287 * t1873;
    let t28866 = 2.0_f64 * t652 * t28864;
    (t28852, t28855, t28860, t28861, t28863, t28864, t28866)
}
