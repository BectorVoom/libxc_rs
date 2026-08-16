//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 160/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk160(t236: f64, t676: f64, t515: f64, t664: f64, t653: f64, t657: f64, t659: f64, t662: f64) -> (f64, f64, f64) {
    let t677 = t676 * t236;
    let t687 = t515 * t664;
    let t698 = -0.99785347515531738034e-2_f64 * t653 + 0.22728884711871118108e-2_f64 * t657 - 0.13276154105060581339e-3_f64 * t659 + 0.3024012879486021305e-4_f64 * t662;
    (t677, t687, t698)
}
