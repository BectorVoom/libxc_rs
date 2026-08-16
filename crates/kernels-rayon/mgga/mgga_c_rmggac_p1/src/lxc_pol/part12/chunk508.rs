//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 508/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk508(t201: f64, t5530: f64, t1451: f64, t457: f64, t1162: f64, t597: f64, t1165: f64, t461: f64) -> (f64, f64, f64, f64, f64) {
    let t5531 = t5530 * t201;
    let t5533 = t1451 * t457;
    let t5538 = t597 * t1162;
    let t5540 = t597 * t1165;
    let t5542 = t201 * t461;
    (t5531, t5533, t5538, t5540, t5542)
}
