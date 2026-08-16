//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 112/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk112(t396: f64, t405: f64, t139: f64, t201: f64, t79: f64) -> (f64, f64, f64, f64) {
    let t408 = 1.0_f64 + 0.5397236614853195164e-1_f64 * t396 * t405;
    let t409 = f64::ln(t408);
    let t411 = 1.0_f64 + 0.193e0_f64 * t409;
    let t412 = 1.0_f64 / t411;
    let t415 = t139 * t201 * t79;
    (t408, t411, t412, t415)
}
