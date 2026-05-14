//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 772/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk772<F: Float>(t2308: F, t2313: F, t3317: F, t3359: F, t4155: F, t4167: F, t4171: F, t4175: F, t4177: F, t4182: F, t4186: F) -> (F,) {
    let t4242 = -0.1294625e1 * t4167 + 0.258925e1 * t4171 + t2308 - 0.60385e0 * t3317 + 0.905775e0 * t4155 + 0.82524375e-1 * t4175 + 0.16504875e0 * t4177 + t2313 - 0.33114e0 * t3359 + 0.248355e0 * t4182 + 0.248355e0 * t4186;
    (t4242,)
}
