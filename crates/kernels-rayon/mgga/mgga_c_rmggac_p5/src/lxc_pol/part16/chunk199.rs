//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 199/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk199(t235: f64, t511: f64, t653: f64, t657: f64, t659: f64, t662: f64) -> (f64, f64) {
    let t684 = t235 * t511;
    let t698 = -0.99785347515531738034e-2_f64 * t653 + 0.22728884711871118108e-2_f64 * t657 - 0.13276154105060581339e-3_f64 * t659 + 0.3024012879486021305e-4_f64 * t662;
    (t684, t698)
}
