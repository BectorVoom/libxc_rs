//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2933/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2933(t4132: f64, t5599: f64, t689: f64, t14103: f64, t9285: f64, t9674: f64, t13730: f64, t1420: f64, t2782: f64, t13726: f64, t9303: f64, t13725: f64, t1445: f64, t2439: f64) -> (f64, f64, f64, f64, f64) {
    let t47929 = t689 * t5599 * t4132;
    let t47932 = t9674 * t14103 * t9285;
    let t47936 = t2782 * t1420 * t13730;
    let t47938 = t9303 * t13726;
    let t47942 = t2439 * t13725 * t1445;
    (t47929, t47932, t47936, t47938, t47942)
}
