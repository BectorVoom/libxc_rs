//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 769/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk769(t1449: f64, t6061: f64, t729: f64, t762: f64, t713: f64, t7560: f64, t265: f64, t33452: f64, t1901: f64, t193: f64, t33693: f64, t33697: f64, t33701: f64, t33707: f64, t33709: f64, t33712: f64, t33717: f64, t33721: f64, t33725: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t33728 = t6061 * t1449;
    let t33730 = t729 * t762 * t33728;
    let t33734 = t729 * t7560 * t713;
    let t33738 = t729 * t265 * t33452;
    let t33741 = -4.0_f64 / 3.0_f64 * t1901 * t33693 - 4.0_f64 / 3.0_f64 * t1901 * t33697 + t89 * t193 * t33701 / 3.0_f64 - t33707 - 2.0_f64 / 9.0_f64 * t1901 * t33709 + 2.0_f64 / 9.0_f64 * t1901 * t33712 + t1901 * t33717 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t33721 + 2.0_f64 / 3.0_f64 * t446 * t33725 + 2.0_f64 / 3.0_f64 * t446 * t33730 - t446 * t33734 / 3.0_f64 - t446 * t33738 / 3.0_f64;
    (t33728, t33730, t33734, t33738, t33741)
}
