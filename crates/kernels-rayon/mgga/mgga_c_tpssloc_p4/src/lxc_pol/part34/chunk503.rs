//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 503/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk503(t1709: f64, t3431: f64, t1174: f64, t3439: f64, t60: f64, t461: f64, t1409: f64, t3450: f64, t3448: f64, t135: f64, t1716: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4896 = t3431 * t1709;
    let t4897 = t1174 * t4896;
    let t4899 = t60 * t3439;
    let t4900 = t4899 * t461;
    let t4904 = t3450 * t1409;
    let t4908 = t3448 * t461;
    let t4916 = t135 * t1716;
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    (t4896, t4897, t4899, t4900, t4904, t4908, t4916, t4917, t4919)
}
