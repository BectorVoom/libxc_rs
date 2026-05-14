//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 944/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk944<F: Float>(t1092: F, t13178: F, t5168: F, t1134: F, t1800: F, t2850: F, t3202: F, t3200: F, t1018: F, t1747: F, t1017: F, t86: F, t3214: F, t3206: F, t4552: F, t4992: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t13179 = t1092 * t13178;
    let t13181 = t5168 * sigma0;
    let t13182 = t13181 * t1134;
    let t13183 = t1092 * t13182;
    let t13186 = t1800 * t2850;
    let t13187 = t3202 * t13186;
    let t13188 = t3200 * t13187;
    let t13190 = t1018 * t1747;
    let t13192 = t86 * t1017 * t13190;
    let t13193 = t13192 * t3214;
    let t13195 = t13192 * t3206;
    let t13199 = t86 * t4992 * t4552;
    (t13179, t13181, t13183, t13188, t13193, t13195, t13199)
}
