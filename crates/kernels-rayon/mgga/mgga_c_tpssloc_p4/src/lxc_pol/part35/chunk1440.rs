//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1440/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1440(t103515: f64, t103528: f64, t103744: f64, t103799: f64, t103810: f64, t15659: f64, t22348: f64, t24589: f64, t24812: f64, t27460: f64, t27489: f64, t27507: f64, t27516: f64, t27536: f64, t27549: f64, t29745: f64, t29776: f64, t29781: f64, t29782: f64, t3624: f64, t3625: f64, t5975: f64, t5979: f64, t6218: f64, t7283: f64, t7362: f64, t7373: f64, t8066: f64, t8082: f64, t85963: f64, t85971: f64, t85972: f64, t95033: f64) -> f64 {
    let t109283 = 0.49348022005446793095e-1_f64 * t24812 * t27489 * t103515 * t15659 - 0.82246703342411321826e-2_f64 * t103744 + 0.54831135561607547884e-2_f64 * t95033 - 0.49348022005446793095e-1_f64 * t7373 * t27536 * t29781 - 0.65797362673929057459e-1_f64 * t27507 * t29745 - 3.0_f64 * t3624 * t8082 * t3625 * t6218 - 0.43864908449286038307e-1_f64 * t103799 - 0.82246703342411321826e-2_f64 * t7283 * t7362 * t27460 * t5979 - 0.16449340668482264365e-1_f64 * t7283 * t7362 * t27460 * t5975 - 0.16449340668482264365e-1_f64 * t103810 - 0.13159472534785811492e0_f64 * t27507 * t29782 + 0.82246703342411321826e-2_f64 * t24589 * t103528 * t8066 - 0.10966227112321509577e-1_f64 * t27549 * t27516 * t29776 - 0.49348022005446793095e-1_f64 * t85963 * t85971 * t22348 * t85972;
    t109283
}
