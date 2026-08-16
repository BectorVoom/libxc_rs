//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 207/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk207(t531: f64, t993: f64, t569: f64, t986: f64, t568: f64, t600: f64, t1000: f64, t193: f64, t557: f64, t574: f64, t597: f64, t902: f64, t915: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1004 = t531 * t993;
    let t1007 = t569 * t986;
    let t1008 = t568 * t1007;
    let t1012 = t600 * t986;
    let t1013 = t568 * t1012;
    let t1016 = 0.35750489951850426669e0_f64 * t1000 * t193 + 0.29792074959875355558e-1_f64 * t902 - 0.35750489951850426669e0_f64 * t557 * t1004 - 0.23005755572352449806e1_f64 * t574 * t1008 - 0.19171462976960374838e0_f64 * t915 + 0.23005755572352449806e1_f64 * t597 * t1013;
    (t1004, t1007, t1008, t1012, t1013, t1016)
}
