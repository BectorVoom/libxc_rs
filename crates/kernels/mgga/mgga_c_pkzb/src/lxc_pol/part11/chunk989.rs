//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 989/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk989<F: Float>(t1978: F, t17402: F, t17348: F, t5870: F, t690: F, t1936: F, t239: F, t1939: F, t5801: F, t659: F, t1853: F, t1897: F, t212: F, t1900: F, t1914: F, t5498: F, t709: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17477 = t1978 * t1978;
    let t17478 = 1.0 / t17477;
    let t17487 = 0.16979925925925925926e1 * t17402;
    let t17505 = 0.5356037037037037037e1 * t17348;
    let t17514 = t690 * t5870;
    let t17517 = t1936 * t1936;
    let t17519 = t239 / t17517;
    let t17520 = t1939 * t1939;
    let t17521 = 1.0 / t17520;
    let t17536 = t659 * t5801;
    let t17541 = t212 / t1897 / t1853;
    let t17548 = 0.13388493827160493828e1 * t17402;
    let t17566 = 0.31003950617283950618e1 * t17348;
    let t17575 = t1897 * t1897;
    let t17577 = t212 / t17575;
    let t17578 = t1900 * t1900;
    let t17579 = 1.0 / t17578;
    let t17601 = t239 / t1936 / t1914;
    let t17624 = t709 * t5498;
    (t17478, t17487, t17505, t17514, t17519, t17521, t17536, t17541, t17548, t17566, t17577, t17579, t17601, t17624)
}
