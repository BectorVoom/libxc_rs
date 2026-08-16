//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 765/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk765(t345: f64, t7561: f64, t1634: f64, t6705: f64, t6704: f64, t1603: f64, t1945: f64, t1409: f64, t3: f64, t1933: f64, t1597: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7562 = t345 * t7561;
    let t7565 = t6705 * t1634;
    let t7566 = t6704 * t7565;
    let t7569 = t1603 * t1945;
    let t7573 = t3 * t1409;
    let t7574 = t1933 * t7573;
    let t7577 = t1597 * t343;
    (t7562, t7565, t7566, t7569, t7573, t7574, t7577)
}
