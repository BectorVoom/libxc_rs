//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 861/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk861<F: Float>(t5803: F, t56: F, t5980: F, t38: F, t370: F, t3577: F, t3603: F, t2209: F, t780: F, t2715: F, t342: F, t2712: F, t35: F, t3521: F, t3523: F, t3525: F, t3531: F, t3569: F, t3583: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7013 = 0.6495611111111111 * t5803;
    let t7015 = t56 * t5980;
    let t7017 = 2.923025 * t38 * t7015;
    let t7018 = t370 * t5980;
    let t7024 = 0.3247805555555556 * t3577;
    let t7026 = 0.6495611111111111 * t3603;
    let t7027 = t780 * t2209;
    let t7031 = t2715 * t342;
    let t7035 = t2712 * t342;
    let t7039 = -t3521 - t3523 + t3525 - 2.0 / 9.0 * t3531 - 0.48968 * t3569 + t7024 - 0.97936 * t3583 - t7026 + 3.0 * t360 * t35 * t7027 + 3.0 / 2.0 * t360 * t35 * t7031 - 6.0 * t360 * t35 * t7035;
    (t7013, t7015, t7017, t7018, t7024, t7026, t7027, t7031, t7035, t7039)
}
