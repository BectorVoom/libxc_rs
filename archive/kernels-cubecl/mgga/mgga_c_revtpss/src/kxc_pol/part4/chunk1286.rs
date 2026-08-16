//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1286/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1286<F: Float>(t1651: F, t3133: F, t1045: F, t3117: F, t12167: F, t15905: F, t11631: F, t3151: F, t15907: F, t3057: F, t380: F, t3088: F, t370: F) -> (F, F, F, F, F, F) {
    let t16076 = t1651 * t3133;
    let t16077 = t16076 * t1045;
    let t16078 = t3117 * t16077;
    let t16081 = t12167 * t15905;
    let t16082 = t11631 * t3151;
    let t16083 = t15907 * t16082;
    let t16084 = t3117 * t16083;
    let t16087 = t3057 * t380;
    let t16088 = t3088 * t370;
    (t16076, t16078, t16081, t16084, t16087, t16088)
}
