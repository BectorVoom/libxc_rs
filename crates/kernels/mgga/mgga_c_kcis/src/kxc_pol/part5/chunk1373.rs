//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1373/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1373<F: Float>(t12361: F, t15008: F, t17676: F, t17685: F, t17686: F, t22151: F, t22582: F, t22585: F, t22588: F, t22592: F, t22596: F, t22601: F, t22604: F, t22607: F, t22610: F, t22616: F, t22619: F, t22623: F, t4202: F, t5133: F, t5459: F, t5947: F, t7237: F) -> F {
    let t22627 = F::new(0.26531111111111111111e0) * t5133 * t22582 - F::new(0.11791604938271604938e0) * t5133 * t22585 + F::new(0.17687407407407407407e0) * t15008 * t22588 + F::new(0.10612444444444444444e0) * t5133 * t22592 - F::new(0.88437037037037037037e-1) * t5133 * t22596 - F::new(0.1857375e-1) * t4202 * t22151 - F::new(0.15918666666666666667e0) * t5133 * t22601 - F::new(0.21224888888888888889e0) * t15008 * t22604 + F::new(0.53062222222222222222e-1) * t5133 * t22607 - F::new(0.44218518518518518518e-1) * t5133 * t22610 - F::new(0.1857375e-1) * t12361 * t7237 + F::new(0.371475e-1) * t4202 * t22616 - F::new(0.9286875e-2) * t5947 * t22619 + F::new(0.11791604938271604938e-1) * t17676 + F::new(0.9286875e-2) * t22623 * t5459 - t17685 + F::new(0.70749629629629629628e-1) * t17686;
    t22627
}
