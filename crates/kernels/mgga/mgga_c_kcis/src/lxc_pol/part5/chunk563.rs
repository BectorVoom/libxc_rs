//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 563/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk563<F: Float>(t2394: F, t41: F, t335: F, t333: F, t1057: F, t733: F, t1065: F, t738: F, t1080: F, t743: F, t113: F, t2844: F) -> (F, F, F, F, F, F) {
    let t3110 = t2394 * t41;
    let t3111 = t3110 * t335;
    let t3113 = F::cast_from(0.16804375e-4_f64) * t333 * t3111;
    let t3114 = t733 * t1057;
    let t3122 = t738 * t1065;
    let t3130 = t743 * t1080;
    let t3150 = t113 * t2844;
    (t3110, t3113, t3114, t3122, t3130, t3150)
}
