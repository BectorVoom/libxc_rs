//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1286/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1286(t1651: f64, t3133: f64, t1045: f64, t3117: f64, t12167: f64, t15905: f64, t11631: f64, t3151: f64, t15907: f64, t3057: f64, t380: f64, t3088: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64) {
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
