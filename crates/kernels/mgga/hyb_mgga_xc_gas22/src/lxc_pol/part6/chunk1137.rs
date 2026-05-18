//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1137/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1137<F: Float>(t11224: F, t11230: F, t11241: F, t11256: F, t541: F, t1175: F, t4485: F, t1563: F, t3656: F, t1528: F, t3792: F, t1115: F, t4583: F) -> (F, F, F, F, F, F) {
    let t11258 = t11224 + t11230 + t11241 + t11256;
    let t11259 = t11258 * t541;
    let t11260 = t4485 * t1175;
    let t11261 = t3656 * t1563;
    let t11263 = t1528 * t3792;
    let t11265 = t1115 * t4583;
    (t11258, t11259, t11260, t11261, t11263, t11265)
}
