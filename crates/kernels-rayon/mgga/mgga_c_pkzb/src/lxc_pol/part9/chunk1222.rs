//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1222/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1222(t20849: f64, t237: f64, t20904: f64, t20913: f64, t20916: f64, t20921: f64, t20924: f64, t21270: f64, t21273: f64, t21275: f64, t21277: f64, t21281: f64) -> (f64, f64) {
    let t21283 = 0.19751673498613801407e-1_f64 * t237 * t20849;
    let t21284 = t20904 - t21270 + t21273 - t21275 - t20913 + t20916 + t20921 + t20924 - t21277 + t21281 + t21283;
    (t21283, t21284)
}
