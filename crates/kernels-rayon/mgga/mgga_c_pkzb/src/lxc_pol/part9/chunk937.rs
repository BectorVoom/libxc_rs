//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 937/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk937(t1107: f64, t5490: f64, t1956: f64, t5493: f64, t730: f64, t2816: f64, t702: f64, t1096: f64, t1932: f64, t1917: f64, t2819: f64, t1940: f64, t2815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7226 = t5490 * t1107;
    let t7227 = t5493 * t1956;
    let t7228 = t7226 * t7227;
    let t7230 = 0.10254018858216406658e4_f64 * t730 * t7228;
    let t7231 = t2816 * t702;
    let t7234 = t1096 * t1932;
    let t7237 = t2819 * t1917;
    let t7240 = t2815 * t1940;
    (t7226, t7227, t7228, t7230, t7231, t7234, t7237, t7240)
}
