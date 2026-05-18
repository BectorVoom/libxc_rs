//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 786/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk786<F: Float>(t2545: F, t2550: F, t3461: F, t3503: F, t4236: F, t4248: F, t4252: F, t4256: F, t4258: F, t4263: F, t4267: F, t987: F) -> (F, F) {
    let t4296 = -F::new(0.17648625e1) * t4248 + F::new(0.3529725e1) * t4252 + t2545 - F::new(0.103295e1) * t3461 + F::new(0.1549425e1) * t4236 + F::new(0.31558125e0) * t4256 + F::new(0.6311625e0) * t4258 + t2550 - F::new(0.41678e0) * t3503 + F::new(0.312585e0) * t4263 + F::new(0.312585e0) * t4267;
    let t4297 = t4296 * t987;
    (t4296, t4297)
}
