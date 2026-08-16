//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 677/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk677<F: Float>(t10924: F, t608: F, t4910: F, t620: F, t342: F, t569: F, t969: F, t119: F, t673: F, t142: F, t1797: F, t10: F, t4594: F) -> (F, F, F, F, F, F, F) {
    let t10925 = t608 * t10924;
    let t10928 = F::cast_from(1.0_f64) / t4910 / t620;
    let t10933 = t342 * t969 * t569;
    let t10934 = F::cast_from(0.28842592592592592592e-1_f64) * t10933;
    let t10935 = t119 * t673;
    let t10939 = t142 * t1797;
    let t10949 = t10 * t4594;
    (t10925, t10928, t10933, t10934, t10935, t10939, t10949)
}
