//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2789/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2789(t10982: f64, t1568: f64, t9646: f64, t252: f64, t2769: f64, t2782: f64, t4533: f64, t886: f64, t10513: f64, t15011: f64, t15030: f64, t15038: f64, t2765: f64, t2772: f64, t41060: f64, t41063: f64, t41067: f64, t4487: f64, t4534: f64, t51227: f64, t51231: f64, t51234: f64, t51237: f64, t51240: f64, t51241: f64) -> f64 {
    let t51246 = t9646 * t1568 * t10982;
    let t51251 = t2782 * t252 * t2769 * t4533 * t886;
    let t51253 = 0.91069445034239308175e-1_f64 * t41060 + 0.32927245914677557992e-1_f64 * t41063 + 0.58544643236296698114e-1_f64 * t41067 + 0.79025390195226139182e1_f64 * t2765 * t15030 + 0.39512695097613069591e1_f64 * t15011 * t2772 + 0.16463622957338778996e-1_f64 * t51227 - 0.19756347548806534796e1_f64 * t10513 * t4534 - 0.58544643236296698113e-1_f64 * t51231 + t51234 + 0.39512695097613069591e1_f64 * t10513 * t4487 - 0.26019841438354088051e-2_f64 * t51237 + t51240 + 0.11708928647259339623e0_f64 * t51241 + 0.39512695097613069591e1_f64 * t2765 * t15038 + 0.19637199382202157274e-3_f64 * t51246 - 0.65854491829355115984e-1_f64 * t51251;
    t51253
}
