//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3299/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3299(t2782: f64, t4086: f64, t543: f64, t86506: f64, t86445: f64, t1399: f64, t14255: f64, t21981: f64, t21990: f64, t47417: f64, t47442: f64, t49276: f64, t49361: f64, t5745: f64, t5755: f64, t6862: f64, t6874: f64, t75252: f64, t820: f64, t86441: f64, t86597: f64) -> f64 {
    let t86604 = t2782 * t4086 * t86506 * t543;
    let t86608 = t2782 * t4086 * t86445 * t543;
    let t86616 = 0.58911598146606471821e-3_f64 * t49361 + 0.39512695097613069591e1_f64 * t820 * t49276 * t6862 - t47417 - 0.29272321618148349057e-1_f64 * t75252 - 0.54878743191129263322e-2_f64 * t86597 + 0.79025390195226139182e1_f64 * t5745 * t21981 * t21990 + 0.16463622957338778997e-1_f64 * t86604 + 0.54878743191129263322e-2_f64 * t86608 - 0.19756347548806534796e1_f64 * t820 * t14255 * t6874 - 0.19756347548806534796e1_f64 * t5755 * t86441 * t1399 + t47442;
    t86616
}
