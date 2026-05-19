//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 411/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk411<F: Float>(t2900: F, t2901: F, t848: F, t2864: F, t2867: F, t2869: F, t2873: F, t2875: F, t2877: F) -> (F, F) {
    let t2903 = t2900 * t2901 * t848;
    let t2912 = -F::cast_from(0.57538888888888888889e0_f64) * t2864 + F::cast_from(0.11507777777777777778e1_f64) * t2867 + F::cast_from(0.40256666666666666667e0_f64) * t2869 + F::new(0.366775e-1) * t2873 + F::new(0.73355e-1) * t2875 + F::new(0.137975e0) * t2877;
    (t2903, t2912)
}
