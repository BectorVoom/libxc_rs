//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 246/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk246(t1161: f64, t317: f64, t305: f64, t306: f64) -> (f64, f64, f64, f64, f64) {
    let t1162 = 0.17808333333333333333e-1_f64 * t1161;
    let t1170 = t317 * t317;
    let t1171 = 1.0_f64 / t1170;
    let t1172 = t305 * t1171;
    let t1173 = 1.0_f64 / t306;
    (t1162, t1170, t1171, t1172, t1173)
}
