//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1267/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1267<F: Float>(t16954: F, t16995: F, t17029: F, t17157: F, t300: F, t3535: F, t5192: F, t1179: F, t1188: F, t17150: F, t1196: F, t3531: F, t5207: F, t16783: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t17094: F) -> (F, F, F, F, F) {
    let t17160 = t300 * (t16954 + t16995 + t17029 + t17157);
    let t17162 = 0.11696447245269292414e1 * t5192 * t3535;
    let t17164 = t1179 * t17150 * t1188;
    let t17166 = 0.5848223622634646207e0 * t1196 * t17164;
    let t17168 = 0.34631718211362927518e2 * t3531 * t5207;
    let t17169 = -t16783 - t16786 - t16788 - t16790 - t16809 - t16814 + t16834 + t16837 + t16839 + t16842 + t16844 + t16846 + t16945 + t17160 + t17162 - t17094 - t17166 - t17168;
    (t17160, t17162, t17166, t17168, t17169)
}
