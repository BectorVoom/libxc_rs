//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1038/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1038(t10344: f64, t10356: f64, t11483: f64, t11501: f64, t11507: f64, t11510: f64, t11520: f64, t11524: f64, t11527: f64, t1250: f64, t3259: f64, t3273: f64, t3914: f64, t3920: f64, t3923: f64, t397: f64, t6555: f64, t6569: f64, t6590: f64, t8546: f64, t8554: f64, t943: f64) -> f64 {
    let t11532 = 0.39512695097613069591e1_f64 * t6555 * t11501 + 0.39512695097613069591e1_f64 * t8546 * t3914 + 0.39512695097613069591e1_f64 * t3259 * t11507 - 0.39512695097613069591e1_f64 * t6569 * t11510 + 0.19756347548806534796e1_f64 * t10356 * t1250 + 0.19756347548806534796e1_f64 * t3273 * t3920 - 0.19756347548806534796e1_f64 * t8554 * t3923 + 0.65854491829355115987e0_f64 * t943 * t11520 - 0.19756347548806534796e1_f64 * t10344 * t11524 + 0.65854491829355115987e0_f64 * t6590 * t11527 + 0.65854491829355115987e0_f64 * t397 * t11483;
    t11532
}
