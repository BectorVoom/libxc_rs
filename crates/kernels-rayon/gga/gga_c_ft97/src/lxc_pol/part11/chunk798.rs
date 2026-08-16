//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 798/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk798(t10799: f64, t871: f64, t296: f64, t1882: f64, t2846: f64, t10735: f64, t10738: f64, t10741: f64, t10745: f64, t10749: f64, t10750: f64, t10752: f64, t10755: f64, t10760: f64, t10765: f64, t10769: f64, t10771: f64, t10773: f64, t446: f64) -> (f64, f64, f64) {
    let t10800 = t871 * t10799;
    let t10801 = t296 * t10800;
    let t10804 = t1882 * t2846;
    let t10806 = -4.0_f64 / 9.0_f64 * t10735 - t446 * t10738 - t446 * t10741 / 3.0_f64 - t10745 / 3.0_f64 - t10749 + t10750 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t10752 + 2.0_f64 * t446 * t10755 - 10.0_f64 / 81.0_f64 * t446 * t10760 - 2.0_f64 * t446 * t10765 - t446 * t10769 + t10771 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t10773 - t446 * t10801 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t10804;
    (t10800, t10801, t10806)
}
