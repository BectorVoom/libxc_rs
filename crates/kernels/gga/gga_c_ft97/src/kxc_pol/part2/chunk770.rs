//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 770/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk770<F: Float>(t10952: F, t10962: F, t10965: F, t10967: F, t10970: F, t12067: F, t1538: F, t1761: F, t1920: F, t3109: F, t3289: F, t438: F, t497: F, t948: F, t984: F) -> F {
    let t12068 = -t1538 * t984 - t1761 * t984 - t1920 * t948 - F::cast_from(2.0_f64) * t3109 * t497 - F::cast_from(2.0_f64) * t3289 * t438 - F::cast_from(12.0_f64) * t10952 + F::cast_from(8.0_f64) * t10962 + F::cast_from(4.0_f64) * t10965 + F::cast_from(8.0_f64) * t10967 + F::cast_from(4.0_f64) * t10970 + t12067;
    t12068
}
