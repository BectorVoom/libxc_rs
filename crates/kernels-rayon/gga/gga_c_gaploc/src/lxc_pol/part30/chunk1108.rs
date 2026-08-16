//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1108/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1108(t1423: f64, t2563: f64, t10023: f64, t2021: f64, t7339: f64, t7372: f64, t3296: f64, t6100: f64, t21451: f64, t2365: f64, t6111: f64, t1967: f64, t21455: f64, t7810: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28889 = t1423 * t2563;
    let t28891 = 0.3575048995185042667e0_f64 * t10023 * t28889;
    let t28915 = 0.59584149919750711116e-1_f64 * t2021 * t7339 * t7372;
    let t28916 = t6100 * t3296;
    let t28917 = 0.38342925953920749676e0_f64 * t28916;
    let t28920 = 0.11916829983950142223e0_f64 * t6111 * t2365 * t21451;
    let t28936 = t7810 * t1967 * t883 * t21455;
    (t28889, t28891, t28915, t28917, t28920, t28936)
}
