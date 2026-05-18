//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1311/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1311<F: Float>(t1167: F, t3199: F, t11422: F, t2099: F, t6516: F, t178: F, t28287: F, t11395: F, t18657: F, t2380: F, t11477: F, t3185: F, t6475: F) -> (F, F, F, F, F) {
    let t31790 = t1167 * t3199;
    let t31805 = t6516 * t2099 * t11422;
    let t31807 = t28287 * t178;
    let t31817 = t2380 * t18657 * t11395;
    let t31820 = t3185 * t6475 * t11477;
    (t31790, t31805, t31807, t31817, t31820)
}
