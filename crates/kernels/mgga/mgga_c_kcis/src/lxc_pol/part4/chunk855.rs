//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 855/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk855<F: Float>(t187: F, t2709: F, t8631: F, t8634: F, t8637: F, t867: F, t8682: F, t8700: F, t8704: F, t8708: F, t8713: F, t8725: F, t8737: F, t8745: F, t8849: F, t8893: F) -> (F,) {
    let t8912 = t187 * (t8849 + t8893) - 0.51947267698127589897e2 * t867 * t8713 + 0.1038945353962551798e3 * t867 * t8682 - 0.58482233974552040708e0 * t867 * t8700 - 0.21687161765563048428e-1 * t2709 * t8634 + 0.16265371324172286321e-1 * t2709 * t8637 - t8725 + t8737 + t8745 - 0.35089340384731224426e1 * t867 * t8704 + 0.35089340384731224426e1 * t867 * t8708 - 0.32530742648344572643e-1 * t2709 * t8631;
    (t8912,)
}
