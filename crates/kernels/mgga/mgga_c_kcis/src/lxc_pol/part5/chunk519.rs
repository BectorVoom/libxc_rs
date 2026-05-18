//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 519/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk519<F: Float>(t160: F, t2466: F, t2471: F, t2475: F, t740: F, t122: F, t2379: F, t2434: F, t2439: F, t2440: F, t2459: F, t60: F, t684: F, t718: F, t728: F, t745: F, t97: F) -> (F, F) {
    let t2477 = -F::new(0.11955719325063177623e-1) * t740 + F::new(0.40985e-2) * t2466 - F::new(0.10566666666666666667e-2) * t2471 + F::new(0.3884654180847230157e-4) * t160 - F::new(0.420109375e-5) * t2475;
    let t2479 = F::new(0.23426533963880895498e-2) * t740 * t97 + F::new(0.46853067927761790996e-2) * t2434 * t728 + F::new(0.70279601891642686494e-2) * t2439 * t2440 - F::new(0.23426533963880895498e-2) * t718 * t2459 - t2379 * t122 - F::new(2.0) * t684 * t745 - t60 * t2477;
    (t2477, t2479)
}
