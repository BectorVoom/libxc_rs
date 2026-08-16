//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3064/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3064(t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63323: f64) -> f64 {
    let t63665 = -0.12361111111111111111e-1_f64 * t63291 + 0.37083333333333333334e-1_f64 * t63296 + 0.18541666666666666667e-1_f64 * t63300 + 0.55625000000000000001e-1_f64 * t63304 + 0.41203703703703703704e-2_f64 * t63306 - 0.68672839506172839506e-2_f64 * t63308 - 0.12361111111111111111e-1_f64 * t63313 - 0.61805555555555555555e-2_f64 * t63317 + 0.16481481481481481482e-1_f64 * t50826 - 0.61805555555555555556e-2_f64 * t50828 - 0.19228395061728395062e-1_f64 * t50834 + 0.41203703703703703704e-1_f64 * t63323;
    t63665
}
