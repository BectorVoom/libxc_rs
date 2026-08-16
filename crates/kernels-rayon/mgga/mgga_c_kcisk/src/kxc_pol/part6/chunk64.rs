//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 64/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk64(t60: f64, t180: f64, t182: f64, t183: f64) -> (f64, f64) {
    let t187 = t60 * t60;
    let t189 = 0.19711288999999999999e-2_f64 * t180 * t182 * t183 - 2.0_f64 * t187;
    let t190 = 1.0_f64 / t189;
    (t189, t190)
}
