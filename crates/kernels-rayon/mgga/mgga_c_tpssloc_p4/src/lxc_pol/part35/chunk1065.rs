//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1065/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1065(t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t21781: f64, t21783: f64, t21786: f64, t21789: f64, t21792: f64, t21795: f64, t21802: f64, t21804: f64) -> f64 {
    let t21885 = 0.6311625e0_f64 * t21781 + 0.3529725e1_f64 * t21783 + 0.264729375e1_f64 * t21786 - 0.20839e0_f64 * t21789 + 0.62517e0_f64 * t21792 + 0.104195e0_f64 * t21795 + 0.57386111111111111112e0_f64 * t21760 - 0.20659e1_f64 * t21764 + 0.309885e1_f64 * t21771 + 0.516475e0_f64 * t21778 + 0.46308888888888888889e-1_f64 * t21802 - 0.157790625e0_f64 * t21804 - 0.103295e1_f64 * t21767 + 0.309885e1_f64 * t21774;
    t21885
}
