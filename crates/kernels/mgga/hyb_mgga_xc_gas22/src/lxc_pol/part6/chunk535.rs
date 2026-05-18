//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 535/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk535<F: Float>(t2539: F, t987: F, t2454: F, t2502: F, t2457: F, t2468: F, t2486: F, t2491: F, t2497: F, t2499: F, t2505: F, t2509: F, t2513: F) -> (F, F, F, F) {
    let t2540 = t2539 * t987;
    let t2545 = F::new(0.68863333333333333333e0) * t2454;
    let t2550 = F::new(0.17365833333333333333e0) * t2502;
    let t2554 = -F::new(0.17648625e1) * t2486 + F::new(0.3529725e1) * t2491 + t2545 - F::new(0.103295e1) * t2457 + F::new(0.1549425e1) * t2468 + F::new(0.31558125e0) * t2497 + F::new(0.6311625e0) * t2499 + t2550 - F::new(0.41678e0) * t2505 + F::new(0.312585e0) * t2509 + F::new(0.312585e0) * t2513;
    (t2540, t2545, t2550, t2554)
}
