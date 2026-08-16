//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1972/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1972(t87403: f64, t87405: f64, t87414: f64, t87425: f64, t87432: f64, t92679: f64, t98818: f64, t98820: f64, t98822: f64, t98824: f64, t98826: f64, t98828: f64, t98830: f64, t98833: f64, t98836: f64, t98838: f64, t98842: f64, t98844: f64) -> f64 {
    let t101486 = 119.0_f64 / 1728.0_f64 * t87403 - 0.21083550404717759668e-2_f64 * t87405 + t92679 - t98818 / 192.0_f64 - t98820 / 192.0_f64 - t98822 / 96.0_f64 - t98824 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t98826 - 35.0_f64 / 288.0_f64 * t98828 + 7.0_f64 / 144.0_f64 * t98830 - t98833 / 192.0_f64 - t87414 - 0.56521858531796547194e-2_f64 * t98836 - 0.23739180583354549821e0_f64 * t87425 - 0.45217486825437237755e-1_f64 * t87432 - 0.33913115119077928317e-1_f64 * t98838 - 0.24223653656484234512e-2_f64 * t98842 + t98844 / 96.0_f64;
    t101486
}
