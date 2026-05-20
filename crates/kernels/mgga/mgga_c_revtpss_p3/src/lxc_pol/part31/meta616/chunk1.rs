//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2062/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2062<F: Float>(t99029: F, t25266: F, t4426: F, t1561: F, t93048: F, t14741: F, t1945: F, t807: F, t10886: F, t4416: F, t7028: F, t1549: F, t92968: F) -> (F, F, F, F, F, F) {
    let t99030 = F::cast_from(0.28582678745379824648e-4_f64) * t99029;
    let t99033 = t25266 * t4426;
    let t99034 = F::cast_from(0.40015750243531754508e-2_f64) * t99033;
    let t99035 = t93048 * t1561;
    let t99041 = t807 * t1945 * t14741;
    let t99042 = F::cast_from(0.11433071498151929859e-3_f64) * t99041;
    let t99044 = t10886 * t7028 * t4416;
    let t99050 = t92968 * t1549;
    (t99030, t99034, t99035, t99042, t99044, t99050)
}
