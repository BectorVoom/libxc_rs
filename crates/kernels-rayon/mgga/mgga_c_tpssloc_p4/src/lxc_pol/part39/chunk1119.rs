//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1119/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1119(t1041: f64, t13950: f64, t10370: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t10390: f64, t13750: f64, t13751: f64, t13758: f64, t13762: f64, t13767: f64, t13942: f64, t13946: f64, t13948: f64, t3070: f64, t378: f64, t4579: f64) -> f64 {
    let t13952 = t1041 * t13950 / 3456.0_f64;
    let t13953 = -t13750 + 19.0_f64 / 1728.0_f64 * t13751 * t378 + t10370 / 4608.0_f64 + t10372 / 1296.0_f64 + t10377 + t10381 / 81.0_f64 + t10385 + t13758 + t10390 * t4579 / 2304.0_f64 + t3070 * t13762 / 2304.0_f64 + t13767 + t13942 * t378 / 3072.0_f64 - t13946 + t13948 + t13952;
    t13953
}
