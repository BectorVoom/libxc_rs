//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1425/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1425(t13547: f64, t18176: f64, t3: f64, t1455: f64, t1921: f64, t571: f64, t5808: f64, t1518: f64, t2327: f64, t116: f64, t4292: f64, t670: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18177 = t13547 + t18176;
    let t18178 = t3 * t18177;
    let t18184 = 2.0_f64 * t1455 * t1921;
    let t18186 = 2.0_f64 * t571 * t5808;
    let t18190 = param_d * t18177;
    let t18204 = t2327 * t1518;
    let t18207 = t116 * t4292;
    let t18208 = t18207 * t670;
    (t18178, t18184, t18186, t18190, t18204, t18208)
}
