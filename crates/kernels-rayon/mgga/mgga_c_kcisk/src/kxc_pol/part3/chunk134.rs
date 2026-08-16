//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 134/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk134(t457: f64, t460: f64, t338: f64, t456: f64) -> (f64, f64, f64) {
    let t461 = t457 * t460;
    let t464 = t338 * t338;
    let t466 = 0.98556445e-3_f64 * t456 * t461 - 2.0_f64 * t464;
    let t467 = 1.0_f64 / t466;
    (t461, t466, t467)
}
