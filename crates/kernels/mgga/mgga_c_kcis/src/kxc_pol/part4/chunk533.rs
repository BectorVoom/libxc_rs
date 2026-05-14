//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 533/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk533<F: Float>(t1022: F, t2845: F, t1021: F, t2842: F, t1071: F, t2630: F) -> (F, F, F, F) {
    let t2846 = t1022 * t2845;
    let t2847 = t1021 * t2846;
    let t2848 = t2842 * t2847;
    let t2850 = t1071 * t2630;
    (t2846, t2847, t2848, t2850)
}
