//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2641/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2641(t10535: f64, t136: f64, t2457: f64, t6017: f64, t10542: f64, t18726: f64, t2439: f64, t2440: f64, t6072: f64, t2444: f64, t689: f64, t15003: f64, t51258: f64) -> (f64, f64, f64, f64, f64) {
    let t62999 = t10535 * t6017 * t136 * t2457;
    let t63015 = t10542 * t18726;
    let t63050 = t2439 * t2440 * t6072;
    let t63053 = t689 * t2444 * t6072;
    let t63058 = t51258 * t15003;
    (t62999, t63015, t63050, t63053, t63058)
}
