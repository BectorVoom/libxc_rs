//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1358/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1358<F: Float>(t16954: F, t16995: F, t17029: F, t17157: F, t300: F, t3535: F, t5192: F, t1179: F, t1188: F, t17150: F, t1196: F, t3531: F, t5207: F) -> (F, F, F, F) {
    let t17160 = t300 * (t16954 + t16995 + t17029 + t17157);
    let t17162 = F::new(0.11696447245269292414e1) * t5192 * t3535;
    let t17164 = t1179 * t17150 * t1188;
    let t17166 = F::new(0.5848223622634646207e0) * t1196 * t17164;
    let t17168 = F::new(0.34631718211362927518e2) * t3531 * t5207;
    (t17160, t17162, t17166, t17168)
}
