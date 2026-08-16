//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1072/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1072(t5075: f64, t512: f64, t83: f64, t1511: f64, t5336: f64, t204: f64, t99: f64, t4888: f64, t5029: f64, t4892: f64, t5052: f64, t496: f64, t5076: f64) -> (f64, f64, f64, f64, f64) {
    let t16897 = t83 * t512 * t5075;
    let t16901 = t1511 * t5336;
    let t16903 = t99 * t204;
    let t16906 = 0.1301229756036208781e0_f64 * t16903 * t5029 * t4888;
    let t16909 = 0.19263893255070628431e1_f64 * t16903 * t5052 * t4892;
    let t16910 = t496 * t5076;
    (t16897, t16901, t16906, t16909, t16910)
}
