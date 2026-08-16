//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 878/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk878(t2035: f64, t2047: f64, t556: f64, t7257: f64, t572: f64, t1533: f64, t555: f64, t7202: f64, t583: f64, t578: f64, t2051: f64, t2062: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7280 = t2035 * t2047;
    let t7282 = t556 * t7257;
    let t7283 = t572 * t7282;
    let t7284 = t1533 * t7283;
    let t7286 = t555 * t7202;
    let t7287 = t583 * t7286;
    let t7288 = t578 * t7287;
    let t7290 = t2051 * t2062;
    (t7280, t7282, t7283, t7284, t7287, t7288, t7290)
}
