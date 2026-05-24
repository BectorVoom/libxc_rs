//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 672/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk672<F: Float>(t140: F, t3737: F, t4594: F, t139: F, t172: F, t79: F, t721: F, t167: F, t3281: F, t1394: F, t298: F, t569: F) -> (F, F, F, F, F, F, F) {
    let t10494 = t140 * t3737 * t4594;
    let t10500 = t139 * t172 * t79;
    let t10501 = t10500 * t721;
    let t10502 = F::cast_from(0.73697530864197530862e-3_f64) * t10501;
    let t10519 = F::new(6.0) * t167;
    let t10520 = F::new(6.0) * t3281;
    let t10568 = t298 * t1394 * t569;
    (t10494, t10500, t10501, t10502, t10519, t10520, t10568)
}
