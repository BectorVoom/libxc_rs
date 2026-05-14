//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1207/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1207<F: Float>(t16712: F, t1729: F, t2439: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16869: F, t16871: F, t1145: F, t16742: F, t141: F, t16733: F) -> (F, F, F, F) {
    let t16873 = 0.19931111111111111111e0 * t16712;
    let t16876 = t2439 * t1729;
    let t16883 = -t16869 + 0.82156666666666666667e-1 * t16871 - t16873 + 0.29896666666666666667e0 * t16748 + 0.13287407407407407408e0 * t16706 + 0.91285185185185185185e-1 * t16876 + 0.66437037037037037038e-1 * t12299 + 0.26574814814814814816e0 * t12297 - 0.19931111111111111111e0 * t12301 - 0.99655555555555555557e-1 * t12303 - 0.39862222222222222222e0 * t16727;
    let t16886 = t1145 * t16742;
    let t16887 = t141 * t16886;
    let t16889 = t1145 * t16733;
    (t16876, t16883, t16887, t16889)
}
