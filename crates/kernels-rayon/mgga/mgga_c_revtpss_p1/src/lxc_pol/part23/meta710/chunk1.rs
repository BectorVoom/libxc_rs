//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2467/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2467(t14293: f64, t9664: f64, t14103: f64, t9285: f64, t9674: f64, t13726: f64, t9303: f64, t13725: f64, t1445: f64, t2439: f64, t14082: f64, t3920: f64) -> (f64, f64, f64, f64, f64) {
    let t47920 = t14293 * t9664;
    let t47932 = t9674 * t14103 * t9285;
    let t47938 = t9303 * t13726;
    let t47942 = t2439 * t13725 * t1445;
    let t47944 = t14082 * t3920;
    (t47920, t47932, t47938, t47942, t47944)
}
