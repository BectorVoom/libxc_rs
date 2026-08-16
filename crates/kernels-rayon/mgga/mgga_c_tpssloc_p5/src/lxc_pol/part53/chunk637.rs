//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 637/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk637(t2054: f64, t259: f64, t2597: f64, t2713: f64, t6557: f64, t6569: f64, t6574: f64, t7067: f64, t7069: f64, t7072: f64, t7085: f64, t7087: f64, t7092: f64, t7107: f64, t855: f64, t866: f64) -> f64 {
    let t7109 = -t7067 - 0.3289868133696452873e-1_f64 * t6557 - t7069 + 0.16449340668482264365e-1_f64 * t6569 - 0.16449340668482264365e-1_f64 * t6574 + t7072 * t259 + t7085 * t259 - t7087 * t866 - t2597 * t2054 - t2713 * t2054 + 2.0_f64 * t855 * t7092 - t855 * t7107;
    t7109
}
