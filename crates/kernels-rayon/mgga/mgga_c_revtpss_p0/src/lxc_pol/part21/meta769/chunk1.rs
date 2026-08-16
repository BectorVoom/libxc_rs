//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2724/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2724(t45: f64, t40148: f64, t13312: f64, t706: f64, t750: f64, t40150: f64, t10326: f64, t10356: f64, t11231: f64, t14447: f64, t1490: f64, t2251: f64, t2258: f64, t4227: f64, t4230: f64, t4328: f64, t49889: f64, t606: f64, t766: f64, t80: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t50106 = 24.0_f64 * t40148;
    let t50113 = t706 * t750 * t13312;
    let t50114 = 12.0_f64 * t50113;
    let t50115 = 3.0_f64 * t40150;
    let t50132 = piecewise3(t151, 0.0_f64, -56.0_f64 / 81.0_f64 * t4227 * t10356 + 8.0_f64 / 9.0_f64 * t4230 * t2251 + 8.0_f64 / 9.0_f64 * t1490 * t11231 - 2.0_f64 / 3.0_f64 * t80 * t13312 * t606 - 2.0_f64 / 3.0_f64 * t14447 * t2258 - 2.0_f64 / 9.0_f64 * t4328 * t10326 + 2.0_f64 / 3.0_f64 * t766 * t49889);
    (t50106, t50114, t50115, t50132)
}
