//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1100/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1100(t2099: f64, t5954: f64, t5957: f64, t5933: f64, t5935: f64, t2003: f64, t67: f64, t154: f64, t276: f64, t5635: f64, t465: f64, t179: f64, t1885: f64, t299: f64) -> (f64, f64, f64, f64) {
    let t18158 = t5954 * t2099 * t5957;
    let t18167 = t5933 * t2099 * t5935;
    let t18182 = t67 * t2003;
    let t18185 = t276 * t154 * t18182 * t5635;
    let t18199 = t465 * t2003;
    let t18202 = t299 * t179 * t18199 * t1885;
    (t18158, t18167, t18185, t18202)
}
