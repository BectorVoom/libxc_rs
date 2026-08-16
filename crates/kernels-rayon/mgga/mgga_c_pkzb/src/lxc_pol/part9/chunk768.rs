//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 768/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk768(t5462: f64, t626: f64, t1784: f64, t1792: f64, t1813: f64, t184: f64, t188: f64, t5408: f64, t5420: f64, t5424: f64, t622: f64, t634: f64) -> (f64, f64) {
    let t5463 = t626 * t5462;
    let t5466 = 0.65854491829355115987e0_f64 * t5408 * t188 - 0.19756347548806534796e1_f64 * t1784 * t634 + 0.39512695097613069591e1_f64 * t622 * t1792 - 0.19756347548806534796e1_f64 * t622 * t1813 - 0.39512695097613069591e1_f64 * t184 * t5420 + 0.39512695097613069591e1_f64 * t184 * t5424 - 0.65854491829355115987e0_f64 * t184 * t5463;
    (t5463, t5466)
}
