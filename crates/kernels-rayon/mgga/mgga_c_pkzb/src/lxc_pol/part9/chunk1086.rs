//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1086/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1086(t1853: f64, t1897: f64, t212: f64, t17402: f64, t17348: f64, t1900: f64, t1914: f64, t1936: f64, t239: f64, t5816: f64, t694: f64, t1908: f64, t1937: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17541 = t212 / t1897 / t1853;
    let t17548 = 0.13388493827160493828e1_f64 * t17402;
    let t17566 = 0.31003950617283950618e1_f64 * t17348;
    let t17575 = t1897 * t1897;
    let t17577 = t212 / t17575;
    let t17578 = t1900 * t1900;
    let t17579 = 1.0_f64 / t17578;
    let t17601 = t239 / t1936 / t1914;
    let t17605 = t5816 * t694;
    let t17616 = t1908 * t1937;
    (t17541, t17548, t17566, t17577, t17579, t17601, t17605, t17616)
}
