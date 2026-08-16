//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1668/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1668(t141: f64, t16907: f64, t16708: f64, t16710: f64, t16712: f64, t12296: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64) {
    let t16908 = t141 * t16907;
    let t16915 = 4.0_f64 / 27.0_f64 * t16708;
    let t16916 = 4.0_f64 / 9.0_f64 * t16710;
    let t16917 = 2.0_f64 / 9.0_f64 * t16712;
    let t16926 = -t12296 + 8.0_f64 / 27.0_f64 * t12297 + 2.0_f64 / 27.0_f64 * t12299 - 2.0_f64 / 9.0_f64 * t12301 - t12303 / 9.0_f64 + 4.0_f64 / 27.0_f64 * t16706 + t16915 - t16916 - t16917 + 10.0_f64 / 27.0_f64 * t16717 - 4.0_f64 / 3.0_f64 * t16722 - 4.0_f64 / 9.0_f64 * t16727 - 2.0_f64 / 9.0_f64 * t16731 + 2.0_f64 * t16735 + 4.0_f64 / 3.0_f64 * t16740 + 2.0_f64 / 3.0_f64 * t16744 + t16748 / 3.0_f64;
    (t16908, t16926)
}
