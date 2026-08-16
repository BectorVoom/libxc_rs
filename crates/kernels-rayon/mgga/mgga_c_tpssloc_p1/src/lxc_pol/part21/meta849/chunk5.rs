//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3077/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3077(t423: f64, t63784: f64, t63798: f64, t63811: f64, t63825: f64, t18496: f64, t699: f64, t18517: f64, t18514: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64) -> (f64, f64, f64, f64, f64) {
    let t63829 = 0.621814e-1_f64 * (t63784 + t63798 + t63811 + t63825) * t423;
    let t63841 = t699 * t18496;
    let t63843 = t699 * t18517;
    let t63845 = t699 * t18514;
    let t63847 = -0.39862222222222222222e0_f64 * t63291 + 0.11958666666666666667e1_f64 * t63296 + 0.59793333333333333334e0_f64 * t63300 + 0.17938e1_f64 * t63304 + 0.13287407407407407408e0_f64 * t63306 - 0.22145679012345679012e0_f64 * t63308 - 0.39862222222222222222e0_f64 * t63313 - 0.19931111111111111111e0_f64 * t63317 + 0.5314962962962962963e0_f64 * t50826 - 0.19931111111111111111e0_f64 * t50828 - 0.62007901234567901235e0_f64 * t50834 - 0.48685432098765432099e-1_f64 * t63841 - 0.21908444444444444444e0_f64 * t63843 + 0.36514074074074074074e-1_f64 * t63845;
    (t63829, t63841, t63843, t63845, t63847)
}
