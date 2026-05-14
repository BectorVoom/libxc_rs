//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1127/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1127<F: Float>(t2713: F, t2723: F, t7441: F, t7443: F, t1057: F, t7508: F, t1052: F, t2742: F, t2757: F, t7449: F, t7453: F, t2634: F, t2646: F, t1110: F, t21846: F, t7241: F) -> (F, F, F, F, F, F, F) {
    let t22170 = 0.3103560775156404018e4 * t7441 * t2723 * t7443 * t2713;
    let t22171 = t1057 * t7508;
    let t22173 = t1052 * t7508;
    let t22175 = t2757 * t2742;
    let t22179 = 0.57895126195293126241e3 * t7449 * t7453 * t2713;
    let t22181 = 1.0 / t2634 / t2646;
    let t22185 = 0.12304822629859687989e5 * t1110 * t22181 * t21846 * t7241;
    (t22170, t22171, t22173, t22175, t22179, t22181, t22185)
}
