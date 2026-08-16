//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2023/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2023(t91303: f64, t91305: f64, t91310: f64, t91312: f64, t91327: f64, t91344: f64, t80867: f64, t80870: f64, t80872: f64, t91317: f64, t91319: f64, t91321: f64, t91323: f64, t91330: f64, t91333: f64, t91336: f64, t91340: f64, t91346: f64) -> f64 {
    let t93720 = 7.0_f64 / 576.0_f64 * t91303;
    let t93721 = 119.0_f64 / 3456.0_f64 * t91305;
    let t93722 = 0.13457585364713463618e-3_f64 * t91310;
    let t93723 = 0.10541775202358879834e-2_f64 * t91312;
    let t93731 = 0.80745512188280781706e-3_f64 * t91327;
    let t93736 = 0.56521858531796547194e-2_f64 * t91344;
    let t93738 = -t93720 + t93721 + t93722 - t93723 - 119.0_f64 / 432.0_f64 * t80867 + 7.0_f64 / 144.0_f64 * t80870 + 7.0_f64 / 288.0_f64 * t80872 + 5.0_f64 / 96.0_f64 * t91317 + 5.0_f64 / 96.0_f64 * t91319 + 5.0_f64 / 192.0_f64 * t91321 + 0.20186378047070195426e-3_f64 * t91323 + t93731 + 0.33913115119077928316e-1_f64 * t91330 + 0.16956557559538964158e-1_f64 * t91333 - 0.40372756094140390853e-3_f64 * t91336 + 0.24223653656484234512e-2_f64 * t91340 - t93736 + 0.33643963411783659044e-4_f64 * t91346;
    t93738
}
