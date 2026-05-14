//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 318/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk318<F: Float>(t1110: F, t1112: F, t1023: F, t1028: F, t1050: F, t1054: F, t1059: F, t1067: F, t1068: F, t1102: F, t1109: F, t462: F, t493: F, t865: F, t502: F, t508: F) -> (F, F, F) {
    let t1114 = 0.5848223622634646207e0 * t1110 * t1112;
    let t1115 = t1028 + t1050 + t1054 - t1059 + t462 * t1068 + t1102 + 0.19751673498613801407e-1 * t1067 * t493 - t1109 - t1114 - t865 - t1023;
    let t1117 = t502 * t508;
    (t1114, t1115, t1117)
}
