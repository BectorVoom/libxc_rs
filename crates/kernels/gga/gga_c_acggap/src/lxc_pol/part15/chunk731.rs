//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 731/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk731<F: Float>(t157: F, t406: F, t556: F, t309: F, t525: F, t1603: F, t615: F, t301: F, t560: F, t105: F, t566: F, t95: F, t467: F, t1427: F, t8034: F, t5439: F, t8040: F) -> (F, F, F, F, F, F, F, F) {
    let t9025 = t556 * t406 * t157;
    let t9029 = t525 * t309;
    let t9058 = t615 * t1603;
    let t9089 = t560 * t301;
    let t9096 = t566 * t95 * t105;
    let t9098 = t560 * t467;
    let t9108 = t8034 * t1427;
    let t9114 = t8040 * t5439;
    (t9025, t9029, t9058, t9089, t9096, t9098, t9108, t9114)
}
