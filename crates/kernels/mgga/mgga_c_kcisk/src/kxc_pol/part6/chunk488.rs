//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 488/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk488<F: Float>(t4856: F, t608: F, t606: F, t609: F, t4834: F, t353: F, t579: F, t964: F, t163: F, t657: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4857 = F::new(1.0) / t4856;
    let t4858 = t608 * t4857;
    let t4864 = F::new(1.0) / t609 / t606;
    let t4868 = F::new(4.0) / F::new(9.0) * t4834;
    let t4876 = F::cast_from(0.39862222222222222223e0_f64) * t4834;
    let t4881 = F::new(1.0)/F::sqrt(t606);
    let t4887 = t353 * t964 * t579;
    let t4888 = F::cast_from(0.27385555555555555555e0_f64) * t4887;
    let t4889 = t163 * t657;
    (t4857, t4858, t4864, t4868, t4876, t4881, t4887, t4888, t4889)
}
