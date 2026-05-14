//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 556/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk556<F: Float>(t1106: F, t3255: F, t1098: F, t1111: F, t1116: F, t2840: F, t346: F, t2844: F, t347: F, t1018: F, t245: F) -> (F, F, F, F, F, F) {
    let t3256 = t3255 * t1106;
    let t3258 = t1098 * t1111;
    let t3260 = t1098 * t1116;
    let t3262 = t2840 * t346;
    let t3263 = t347 * t2844;
    let t3268 = t1018 * t245;
    let t3269 = t3268 * t347;
    (t3256, t3258, t3260, t3262, t3263, t3269)
}
