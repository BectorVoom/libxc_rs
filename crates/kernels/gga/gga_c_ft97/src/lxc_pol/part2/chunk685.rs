//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 685/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk685<F: Float>(t10952: F, t10962: F, t10965: F, t10967: F, t10970: F, t12067: F, t1538: F, t1761: F, t1920: F, t3109: F, t3289: F, t438: F, t497: F, t948: F, t984: F, t18: F, t502: F) -> (F, F) {
    let t12068 = -t1538 * t984 - t1761 * t984 - t1920 * t948 - 2.0 * t3109 * t497 - 2.0 * t3289 * t438 - 12.0 * t10952 + 8.0 * t10962 + 4.0 * t10965 + 8.0 * t10967 + 4.0 * t10970 + t12067;
    let t12081 = t502 * t18;
    (t12068, t12081)
}
