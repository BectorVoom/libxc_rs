//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1221/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1221<F: Float>(t2697: F, t32601: F, t3368: F, t1101: F, t119: F, t15660: F, t110958: F, t110962: F, t110965: F, t110969: F, t110972: F, t110975: F, t110978: F, t110981: F, t110983: F, t20: F) -> (F, F) {
    let t110986 = t3368 * t32601 * t2697;
    let t110990 = t1101 * t15660 * t119 * t2697;
    let t110992 = 0.72916666666666666668e-1 * t110958 + 0.35108024691358024692e0 * t110962 - 0.24305555555555555556e0 * t110965 - 0.24305555555555555556e0 * t110969 + 0.55715333333333333331e-1 * t110972 - 0.65001222222222222219e-1 * t110975 - 0.2089325e-1 * t110978 - 0.10416666666666666667e-1 * t110981 + 0.14583333333333333334e0 * t110983 - 0.31250000000000000001e-1 * t110986 - 0.10416666666666666667e-1 * t110990;
    let t110994 = t32601 * t20;
    (t110992, t110994)
}
