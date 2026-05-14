//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 953/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk953<F: Float>(t9011: F, t7037: F, t7040: F, t7043: F, t7082: F, t7089: F, t9008: F, t9029: F, t9149: F, t9152: F, t9155: F, t9159: F, t9147: F, t968: F, t949: F, t9112: F) -> (F, F, F, F, F) {
    let t9161 = 0.59793333333333333334e0 * t9011;
    let t9166 = -0.1898925e1 * t9149 + 0.142419375e1 * t9152 - 0.76790625e-1 * t9155 + 0.39862222222222222223e0 * t9008 + 0.27385555555555555555e0 * t9159 - t9161 + 0.8969e0 * t9029 - t7082 - t7089 + 0.54771111111111111111e0 * t7037 - 0.16431333333333333333e0 * t7040 - 0.16431333333333333333e0 * t7043;
    let t9167 = t9147 + t9166;
    let t9168 = t9167 * t968;
    let t9170 = 1.0 * t949 * t9168;
    let t9171 = 0.33114e0 * t9112;
    (t9161, t9167, t9168, t9170, t9171)
}
