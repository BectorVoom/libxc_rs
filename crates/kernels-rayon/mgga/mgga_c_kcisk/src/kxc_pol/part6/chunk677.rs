//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 677/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk677(t10924: f64, t608: f64, t4910: f64, t620: f64, t342: f64, t569: f64, t969: f64, t119: f64, t673: f64, t142: f64, t1797: f64, t10: f64, t4594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10925 = t608 * t10924;
    let t10928 = 1.0_f64 / t4910 / t620;
    let t10933 = t342 * t969 * t569;
    let t10934 = 0.28842592592592592592e-1_f64 * t10933;
    let t10935 = t119 * t673;
    let t10939 = t142 * t1797;
    let t10949 = t10 * t4594;
    (t10925, t10928, t10933, t10934, t10935, t10939, t10949)
}
