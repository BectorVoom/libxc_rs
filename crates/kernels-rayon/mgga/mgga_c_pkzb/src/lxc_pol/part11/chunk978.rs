//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 978/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk978(t1034: f64, t1044: f64, t10573: f64, t10577: f64, t10621: f64, t10675: f64, t10692: f64, t10696: f64, t164: f64, t167: f64, t1717: f64, t1721: f64, t183: f64, t2682: f64, t2693: f64, t3441: f64, t3460: f64, t5389: f64, t5391: f64, t588: f64) -> f64 {
    let t10727 = -0.39512695097613069591e1_f64 * t5389 * t10692 * t5391 + 0.39512695097613069591e1_f64 * t1717 * t10696 * t1721 + 0.39512695097613069591e1_f64 * t2682 * t10577 + 0.39512695097613069591e1_f64 * t1717 * t10692 * t1721 - 0.19756347548806534796e1_f64 * t588 * t3460 * t1034 * t164 - 0.19756347548806534796e1_f64 * t588 * t1044 * t3441 * t164 - 0.19756347548806534796e1_f64 * t588 * t10696 * t164 - 0.65854491829355115987e0_f64 * t588 * t183 * t10621 * t164 - 0.19756347548806534796e1_f64 * t2693 * t10573 - 0.65854491829355115987e0_f64 * t588 * t10692 * t164 + 0.65854491829355115987e0_f64 * t167 * t10675;
    t10727
}
