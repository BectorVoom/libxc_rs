//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3289/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3289(t14633: f64, t14648: f64, t14659: f64, t14749: f64, t1553: f64, t1555: f64, t18612: f64, t225: f64, t227: f64, t229: f64, t2634: f64, t2638: f64, t2639: f64, t2642: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t61234: f64, t61519: f64, t62259: f64, t62260: f64, t62262: f64, t62263: f64, t62266: f64, t62267: f64, t62287: f64, t62313: f64, t73: f64, t830: f64, t832: f64) -> f64 {
    let t62347 = -(t62259 + t62260 + t62262 + t62263 + t62266 + t62267 + t62287 + t62313) * t225 * t229 + 6.0_f64 * t830 * t18612 - 12.0_f64 * t2634 * t6010 - 24.0_f64 * t227 * t2638 * t61234 - 12.0_f64 * t6006 * t2639 + 12.0_f64 * t4409 * t4420 + 3.0_f64 * t2634 * t6013 + 6.0_f64 * t1553 * t14659 + 3.0_f64 * t227 * t832 * t61519 + 240.0_f64 * t4415 * t14648 * t14749 + 3.0_f64 * t6006 * t2642 + 6.0_f64 * t14633 * t1555 - 48.0_f64 * t4409 * t73 * t4417;
    t62347
}
