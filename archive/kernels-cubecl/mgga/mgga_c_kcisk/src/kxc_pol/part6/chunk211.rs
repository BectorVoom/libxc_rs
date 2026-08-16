//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 211/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk211<F: Float>(t163: F, t80: F, t81: F, t867: F, t869: F, t874: F, t88: F) -> (F, F, F) {
    let t877 = t80 * t81 * t163;
    let t879 = -F::cast_from(0.632975e0_f64) * t867 - F::cast_from(0.29896666666666666667e0_f64) * t869 - F::cast_from(0.1023875e0_f64) * t874 - F::cast_from(0.82156666666666666667e-1_f64) * t877;
    let t880 = F::cast_from(1.0_f64) / t88;
    (t877, t879, t880)
}
