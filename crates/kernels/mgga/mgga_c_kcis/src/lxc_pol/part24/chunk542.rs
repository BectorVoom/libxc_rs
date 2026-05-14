//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 542/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk542<F: Float>(t5127: F, t5162: F, t355: F, t377: F, t4922: F, t381: F, t389: F, t5025: F, t1189: F, t3226: F, t3436: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5163 = t5127 + t5162;
    let t5164 = t5163 * t355;
    let t5165 = t5164 * sigma0;
    let t5166 = t5165 * t377;
    let t5168 = t4922 * t355;
    let t5169 = t5168 * t381;
    let t5170 = t5169 * t389;
    let t5172 = t5025 * t381;
    let t5173 = t5172 * t1189;
    let t5175 = t3226 * t3436;
    (t5163, t5164, t5165, t5166, t5168, t5169, t5170, t5172, t5173, t5175)
}
