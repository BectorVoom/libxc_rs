//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1287/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1287<F: Float>(t16087: F, t16088: F, t4757: F, t906: F, t3092: F, t380: F, t994: F, t606: F, t999: F, t4578: F, t905: F, t1045: F) -> (F, F, F, F, F, F) {
    let t16089 = t16087 * t16088;
    let t16090 = t4757 * t906;
    let t16091 = t3092 * t16090;
    let t16094 = t994 * t380;
    let t16095 = t16094 * t16088;
    let t16096 = t606 * t999;
    let t16097 = t4578 * t16096;
    let t16098 = t3092 * t16097;
    let t16101 = t999 * t905;
    let t16102 = t16101 * t606;
    let t16103 = t1045 * t16102;
    (t16089, t16091, t16095, t16096, t16098, t16103)
}
