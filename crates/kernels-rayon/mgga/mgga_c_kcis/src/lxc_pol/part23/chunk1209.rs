//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1209/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1209(t17388: f64, t28624: f64, t97702: f64, t97704: f64, t97707: f64, t97709: f64, t97711: f64, t97713: f64, t97715: f64, t97717: f64, t97719: f64, t97721: f64, t97723: f64, t97725: f64, t97728: f64, t97730: f64, t97732: f64, t97734: f64, t97736: f64) -> (f64, f64) {
    let t97738 = t28624 * t17388;
    let t97740 = -0.1875e0_f64 * t97702 + 0.4046875e-1_f64 * t97704 + 0.12140625e0_f64 * t97707 - 0.20833333333333333333e-1_f64 * t97709 + 0.625e-1_f64 * t97711 - 0.10791666666666666667e0_f64 * t97713 - 0.125e0_f64 * t97715 - 0.5625e0_f64 * t97717 - 0.125e0_f64 * t97719 + 0.125e0_f64 * t97721 + 0.55555555555555555557e-1_f64 * t97723 - 0.9375e-1_f64 * t97725 + 0.5e0_f64 * t97728 + 0.21583333333333333334e0_f64 * t97730 + 0.625e-1_f64 * t97732 - 0.1875e0_f64 * t97734 + 0.89930555555555555557e-2_f64 * t97736 - 0.4046875e-1_f64 * t97738;
    (t97738, t97740)
}
