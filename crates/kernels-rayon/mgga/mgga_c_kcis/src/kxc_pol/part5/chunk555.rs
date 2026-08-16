//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 555/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk555(t926: f64, t930: f64, t257: f64, t929: f64, t244: f64, t346: f64) -> (f64, f64, f64, f64) {
    let t2933 = t926 * t930;
    let t2936 = t929 * t257;
    let t2937 = 1.0_f64 / t2936;
    let t2938 = t244 * t2937;
    let t2943 = 1.0_f64 / t346;
    (t2933, t2937, t2938, t2943)
}
