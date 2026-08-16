//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 775/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk775(t11921: f64, t828: f64, t1035: f64, t11239: f64, t3143: f64, t1043: f64, t3153: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11922 = t828 * t11921;
    let t12046 = t11239 * t1035;
    let t12077 = t11239 * t3143;
    let t12131 = t1043 * t3153;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t11922, t12046, t12077, t12131, t13269, t13272)
}
