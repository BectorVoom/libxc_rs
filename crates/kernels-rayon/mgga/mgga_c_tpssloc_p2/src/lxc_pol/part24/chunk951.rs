//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 951/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk951(t2884: f64, t307: f64, t302: f64, t10743: f64, t2888: f64, t10294: f64, t10544: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10314: f64, t10320: f64, t10323: f64, t10530: f64, t10538: f64, t10547: f64, t10550: f64) -> (f64, f64, f64) {
    let t10770 = 1.0_f64 / t2884 / t307;
    let t10771 = t302 * t10770;
    let t10772 = t10743 * t2888;
    let t10784 = 0.46308888888888888888e0_f64 * t10294;
    let t10785 = 0.16068111111111111111e1_f64 * t10544;
    let t10789 = -0.103295e1_f64 * t10530 - 0.34731666666666666667e0_f64 * t10296 + 0.20839e0_f64 * t10302 + 0.69463333333333333335e-1_f64 * t10298 - 0.46308888888888888889e-1_f64 * t10307 - 0.104195e0_f64 * t10323 + 0.309885e1_f64 * t10538 - 0.104195e0_f64 * t10314 + 0.62517e0_f64 * t10320 - t10784 - t10785 - 0.52945875e1_f64 * t10547 + 0.94674375e0_f64 * t10550 - 0.41678000000000000001e0_f64 * t10300;
    (t10771, t10772, t10789)
}
