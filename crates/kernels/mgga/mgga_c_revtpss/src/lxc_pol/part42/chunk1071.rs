//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1071/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1071<F: Float>(t11150: F, t3181: F, t11144: F, t11852: F, t3124: F, t4820: F, t1655: F, t697: F, t1011: F, t15688: F, t3299: F, t1678: F, t3057: F, t4930: F, t994: F, t3046: F) -> (F, F, F, F, F, F, F, F) {
    let t16199 = t3181 * t11150;
    let t16208 = t11852 * t11144;
    let t16218 = 0.28582678745379824648e-3 * t3124 * t4820;
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16226 = t3299 * t15688;
    let t16284 = t3057 * t1678;
    let t16302 = t994 * t4930;
    let t16305 = t3046 * t1678;
    (t16199, t16208, t16218, t16220, t16226, t16284, t16302, t16305)
}
