//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1075/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1075<F: Float>(t1853: F, t1897: F, t212: F, t17402: F, t17348: F, t1900: F, t1914: F, t1936: F, t239: F, t5498: F, t709: F, t5490: F) -> (F, F, F, F, F, F, F, F) {
    let t17541 = t212 / t1897 / t1853;
    let t17548 = F::new(0.13388493827160493828e1) * t17402;
    let t17566 = F::new(0.31003950617283950618e1) * t17348;
    let t17575 = t1897 * t1897;
    let t17577 = t212 / t17575;
    let t17578 = t1900 * t1900;
    let t17579 = F::new(1.0) / t17578;
    let t17601 = t239 / t1936 / t1914;
    let t17624 = t709 * t5498;
    let t17633 = t709 * t5490;
    (t17541, t17548, t17566, t17577, t17579, t17601, t17624, t17633)
}
