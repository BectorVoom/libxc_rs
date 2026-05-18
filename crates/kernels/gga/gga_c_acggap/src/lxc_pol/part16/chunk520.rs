//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 520/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk520<F: Float>(t121: F, t3151: F, t126: F, t147: F, t3036: F, t383: F, t174: F, t3037: F, t386: F, t387: F, t1035: F, t996: F) -> (F, F, F, F) {
    let t3206 = t121 * t3151;
    let t3207 = t3206 * t126;
    let t3209 = F::new(455.0) / F::new(1296.0) * t3207 * t147;
    let t3210 = t3036 * t383;
    let t3211 = t174 * t3037;
    let t3213 = t386 * t387 * t3211;
    let t3215 = F::new(0.12862205435420921092e-2) * t3210 * t3213;
    let t3216 = t1035 * t996;
    (t3209, t3213, t3215, t3216)
}
