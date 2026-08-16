//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1288/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1288(t10705: f64, t42023: f64, t275: f64, t2790: f64, t2840: f64, t10704: f64, t41995: f64, t41642: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41831: f64, t41833: f64, t41836: f64, t41839: f64, t41842: f64, t41845: f64) -> (f64, f64, f64) {
    let t42025 = 0.2069040516770936012e4_f64 * t42023 * t10705;
    let t42028 = t275 / t2840 / t2790;
    let t42031 = 0.62071215503128080361e4_f64 * t42028 * t41995 * t10704;
    let t42046 = 0.10954222222222222222e1_f64 * t41831 + 0.13145066666666666666e1_f64 * t41833 - 0.98587999999999999998e0_f64 * t41836 - 0.82156666666666666668e-1_f64 * t41839 + 0.197176e1_f64 * t41842 + 0.49293999999999999999e0_f64 * t41845 + 0.17938e1_f64 * t41642 - 0.79724444444444444446e0_f64 * t41656 - 0.5314962962962962963e0_f64 * t41658 + 0.44291358024691358024e0_f64 * t41660 + 0.39862222222222222223e0_f64 * t41662 - 0.88582716049382716048e0_f64 * t41669 - 0.29896666666666666667e0_f64 * t41673 + 0.15944888888888888889e1_f64 * t41675;
    (t42025, t42031, t42046)
}
