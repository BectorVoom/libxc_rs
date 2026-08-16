//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 944/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk944(t1831: f64, t31176: f64, t22804: f64, t32711: f64, t1799: f64, t22690: f64, t22792: f64, t6950: f64, t22779: f64, t32714: f64, t1814: f64, t31175: f64, t8467: f64) -> (f64, f64, f64, f64, f64) {
    let t120375 = t31176 * t1831;
    let t120383 = t22804 * t32711;
    let t120393 = t22792 * t22690 * t6950 * t1799;
    let t120410 = t22779 * t32714;
    let t120416 = t1814 * t31175 * t8467;
    (t120375, t120383, t120393, t120410, t120416)
}
