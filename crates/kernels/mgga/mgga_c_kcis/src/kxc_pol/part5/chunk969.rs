//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 969/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk969<F: Float>(t10470: F, t361: F, t1127: F, t3245: F, t1138: F, t3329: F, t1140: F, t364: F, t357: F, t359: F, t373: F, t9587: F) -> (F, F, F, F, F, F, F) {
    let t10471 = t10470 * t361;
    let t10472 = F::cast_from(0.73697530864197530862e-3_f64) * t10471;
    let t10473 = t3245 * t1127;
    let t10491 = t1138 * t3329;
    let t10496 = t1140 * t1140;
    let t10497 = F::cast_from(1.0_f64) / t10496;
    let t10498 = t364 * t10497;
    let t10506 = F::cast_from(1.0_f64) / t359 / t357;
    let t10513 = t373 * t9587;
    (t10471, t10472, t10473, t10491, t10498, t10506, t10513)
}
