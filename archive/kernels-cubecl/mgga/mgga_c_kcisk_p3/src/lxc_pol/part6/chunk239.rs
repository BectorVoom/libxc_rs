//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 239/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk239<F: Float>(t1050: F, t210: F, t1033: F, t1037: F, t1040: F, t1043: F, t1046: F) -> (F, F) {
    let t1051 = t210 * t1050;
    let t1053 = t1033 / F::cast_from(8.0_f64) - t1037 / F::cast_from(8.0_f64) - t1040 / F::cast_from(4.0_f64) - t1043 / F::cast_from(64.0_f64) + t1046 / F::cast_from(64.0_f64) + t1051 / F::cast_from(16.0_f64);
    (t1051, t1053)
}
