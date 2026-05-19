//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 993/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk993<F: Float>(t9112: F, t9115: F, t6969: F, t6972: F, t9119: F, t9123: F, t9127: F, t9136: F, t9138: F, t9140: F, t9143: F, t9145: F) -> (F, F, F) {
    let t9217 = F::new(0.41678e0) * t9112;
    let t9218 = F::new(0.41678e0) * t9115;
    let t9229 = -t9217 - t9218 + F::new(0.312585e0) * t9119 + F::new(0.62517e0) * t9123 + F::new(0.312585e0) * t9127 + F::cast_from(0.13772666666666666667e1_f64) * t6969 - F::new(0.516475e0) * t6972 + F::new(0.3529725e1) * t9136 + F::new(0.6311625e0) * t9138 - F::new(0.17648625e1) * t9140 + F::new(0.6311625e0) * t9143 + F::new(0.31558125e0) * t9145;
    (t9217, t9218, t9229)
}
