//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1531/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1531(t23820: f64, t73: f64, t1063: f64, t23485: f64, t247: f64, t3109: f64, t11922: f64, t23993: f64, t3115: f64, t23935: f64, t4899: f64, t15932: f64, t19826: f64) -> (f64, f64, f64, f64, f64) {
    let t79159 = t23820 * t73;
    let t79219 = t1063 * t247 * t3109 * t23485;
    let t79233 = t3115 * t11922 * t23993;
    let t79253 = t4899 * t11922 * t23935;
    let t79290 = t15932 * t19826;
    (t79159, t79219, t79233, t79253, t79290)
}
