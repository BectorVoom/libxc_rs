//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 767/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk767(t164: f64, t167: f64, t1717: f64, t1721: f64, t1753: f64, t1783: f64, t183: f64, t2682: f64, t2693: f64, t5240: f64, t5251: f64, t5367: f64, t5389: f64, t5391: f64, t5407: f64, t5427: f64, t5431: f64, t588: f64, t600: f64, t621: f64) -> f64 {
    let t5462 = -0.39512695097613069591e1_f64 * t5389 * t5427 * t5391 + 0.39512695097613069591e1_f64 * t1717 * t5431 * t1721 + 0.39512695097613069591e1_f64 * t2682 * t5251 + 0.39512695097613069591e1_f64 * t1717 * t5427 * t1721 - 0.19756347548806534796e1_f64 * t588 * t1783 * t600 * t164 - 0.19756347548806534796e1_f64 * t588 * t621 * t1753 * t164 - 0.19756347548806534796e1_f64 * t588 * t5431 * t164 - 0.65854491829355115987e0_f64 * t588 * t183 * t5367 * t164 - 0.19756347548806534796e1_f64 * t2693 * t5240 - 0.65854491829355115987e0_f64 * t588 * t5427 * t164 + 0.65854491829355115987e0_f64 * t167 * t5407;
    t5462
}
