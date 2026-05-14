//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 509/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk509<F: Float>(t165: F, t2531: F, t779: F, t782: F, t826: F, t164: F, t781: F, t142: F, t143: F, t2379: F, t126: F, t684: F, t15: F, t60: F, t762: F, t647: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2532 = t2531 * t165;
    let t2533 = t779 * t782;
    let t2534 = t2533 * t826;
    let t2535 = 2.0 * t2534;
    let t2537 = 1.0 / t781 / t164;
    let t2538 = t142 * t2537;
    let t2539 = t826 * t826;
    let t2540 = t2538 * t2539;
    let t2541 = 2.0 * t2540;
    let t2542 = t2379 * t143;
    let t2545 = t684 * t126;
    let t2546 = t2545 * t15;
    let t2551 = t60 * t762;
    let t2552 = t2551 * t647;
    (t2532, t2533, t2534, t2535, t2537, t2538, t2539, t2540, t2541, t2542, t2545, t2546, t2551, t2552)
}
