//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 591/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk591<F: Float>(t3106: F, t323: F, t325: F, t2394: F, t41: F, t335: F, t333: F, t1057: F, t733: F, t1056: F, t2829: F, t2845: F, t345: F) -> (F, F, F, F, F, F, F) {
    let t3109 = F::cast_from(0.21133333333333333333e-2_f64) * t323 * t3106 * t325;
    let t3110 = t2394 * t41;
    let t3111 = t3110 * t335;
    let t3113 = F::new(0.16804375e-4) * t333 * t3111;
    let t3114 = t733 * t1057;
    let t3116 = t1056 * t2829;
    let t3119 = t345 * t2845;
    (t3109, t3110, t3111, t3113, t3114, t3116, t3119)
}
