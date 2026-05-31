//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 441/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk441<F: Float>(t926: F, t930: F, t257: F, t929: F, t244: F, t346: F) -> (F, F, F, F) {
    let t2933 = t926 * t930;
    let t2936 = t929 * t257;
    let t2937 = F::cast_from(1.0_f64) / t2936;
    let t2938 = t244 * t2937;
    let t2943 = F::cast_from(1.0_f64) / t346;
    (t2933, t2937, t2938, t2943)
}
