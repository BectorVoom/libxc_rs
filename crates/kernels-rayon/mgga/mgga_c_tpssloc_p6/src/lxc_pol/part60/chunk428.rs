//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 428/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk428(t1709: f64, t3431: f64, t1174: f64, t1409: f64, t3450: f64, t135: f64, t1716: f64, t1714: f64, t3448: f64, t1721: f64, t225: f64, t1222: f64, t1731: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4896 = t3431 * t1709;
    let t4897 = t1174 * t4896;
    let t4904 = t3450 * t1409;
    let t4916 = t135 * t1716;
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4945 = t1721 * t225;
    let t4957 = t1731 * t1222;
    (t4897, t4904, t4917, t4919, t4945, t4957)
}
