//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 746/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk746<F: Float>(t60: F, t12630: F, t123: F, t925: F, t3015: F, t896: F, t2994: F, t3006: F, t898: F, t2995: F, t3012: F, t3: F, t74: F, t83: F) -> (F, F, F, F, F, F, F) {
    let t124 = F::new(0.0) < t60;
    let t15268 = piecewise3::<f64>(t124, t12630, -t12630);
    let t15270 = t123 * t925 * t15268;
    let t15274 = t3015 * t896;
    let t15278 = t2994 * t896;
    let t15279 = t898 * t3006;
    let t15283 = t2995 * t896;
    let t15285 = t3012 * t15283 * t898;
    let t15291 = F::new(1.0) / t74 / t83 * t3 / F::new(4.0);
    (t15270, t15274, t15278, t15279, t15283, t15285, t15291)
}
