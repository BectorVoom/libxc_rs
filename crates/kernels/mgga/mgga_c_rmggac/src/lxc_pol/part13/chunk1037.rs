//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1037/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1037<F: Float>(t798: F, t9551: F, t4048: F, t4905: F, t26291: F, t34813: F, t38489: F, t38491: F, t38493: F, t38496: F, t38498: F, t38500: F, t38502: F, t38506: F, t38511: F, t38515: F, t38519: F, t38521: F, t38526: F, t38528: F, t38531: F, t40724: F) -> (F, F, F, F) {
    let t42634 = t9551 * t798;
    let t42637 = t9551 * t4048;
    let t42640 = t9551 * t4905;
    let t42655 = F::new(0.1702583995731913576e-4) * t38489 + F::new(0.5107751987195740728e-4) * t38491 - F::new(0.5107751987195740728e-4) * t38493 - F::new(0.71845450211182851384e0) * t26291 * t42634 - F::new(0.71845450211182851384e0) * t40724 * t42637 + F::new(0.71845450211182851384e0) * t34813 * t42640 + F::new(0.13637330827122670865e0) * t38496 - F::new(0.1702583995731913576e-4) * t38498 + F::new(0.212822999466489197e-4) * t38500 + F::new(0.1702583995731913576e-4) * t38502 + F::new(0.1702583995731913576e-4) * t38506 - F::new(0.3405167991463827152e-4) * t38511 - F::new(0.3405167991463827152e-4) * t38515 - F::new(0.212822999466489197e-4) * t38519 + F::new(0.1702583995731913576e-4) * t38521 - F::new(0.1702583995731913576e-4) * t38526 + F::new(0.1702583995731913576e-4) * t38528 + F::new(0.1702583995731913576e-4) * t38531;
    (t42634, t42637, t42640, t42655)
}
