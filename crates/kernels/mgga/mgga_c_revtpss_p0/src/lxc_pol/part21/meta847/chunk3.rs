//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3178/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3178<F: Float>(t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t58186: F, t58189: F, t58192: F, t58195: F, t58198: F) -> F {
    let t58572 = -F::cast_from(0.26574814814814814816e0_f64) * t43865 + F::cast_from(0.39862222222222222222e0_f64) * t43883 - F::cast_from(0.93011851851851851855e0_f64) * t43888 + F::cast_from(0.39862222222222222224e0_f64) * t43890 + F::cast_from(0.79724444444444444447e0_f64) * t43892 - F::cast_from(0.59793333333333333333e0_f64) * t43894 - F::cast_from(0.99655555555555555557e-1_f64) * t43896 - F::cast_from(0.65725333333333333332e0_f64) * t58186 - F::cast_from(0.82156666666666666668e-1_f64) * t58189 - F::cast_from(0.82156666666666666668e-1_f64) * t58192 - F::new(0.49294e0) * t58195 - F::cast_from(0.27385555555555555556e-1_f64) * t58198;
    t58572
}
