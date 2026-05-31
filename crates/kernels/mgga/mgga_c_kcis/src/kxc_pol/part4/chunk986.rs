//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 986/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk986<F: Float>(t1141: F, t3323: F, t1138: F, t3329: F, t1140: F, t364: F, t357: F, t359: F, t373: F, t9587: F, t1164: F, t3225: F) -> (F, F, F, F, F, F) {
    let t10488 = t3323 * t1141;
    let t10491 = t1138 * t3329;
    let t10496 = t1140 * t1140;
    let t10497 = F::cast_from(1.0_f64) / t10496;
    let t10498 = t364 * t10497;
    let t10506 = F::cast_from(1.0_f64) / t359 / t357;
    let t10513 = t373 * t9587;
    let t10525 = t1164 * t3225;
    (t10488, t10491, t10498, t10506, t10513, t10525)
}
