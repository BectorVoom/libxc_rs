//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2536/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2536(t51973: f64, t52035: f64, t52037: f64, t2852: f64, t373: f64, t2439: f64, t4628: f64, t1606: f64, t9303: f64, t2923: f64, t4587: f64, t11384: f64, t1596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52082 = 4.0_f64 / 9.0_f64 * t51973;
    let t52091 = 8.0_f64 / 9.0_f64 * t52035;
    let t52092 = 8.0_f64 / 27.0_f64 * t52037;
    let t52110 = t373 * t2852;
    let t52126 = t2439 * t4628;
    let t52127 = 0.27595e0_f64 * t52126;
    let t52128 = t9303 * t1606;
    let t52219 = t4587 * t2923;
    let t52224 = t1596 * t11384;
    (t52082, t52091, t52092, t52110, t52126, t52127, t52128, t52219, t52224)
}
