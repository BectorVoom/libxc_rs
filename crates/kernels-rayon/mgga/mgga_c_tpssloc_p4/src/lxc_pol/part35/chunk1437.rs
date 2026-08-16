//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1437/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1437(t103175: f64, t103261: f64, t103286: f64, t103363: f64, t103464: f64, t109060: f64, t1238: f64, t1761: f64, t21510: f64, t2154: f64, t22393: f64, t24589: f64, t24601: f64, t27382: f64, t27549: f64, t27774: f64, t27820: f64, t29678: f64, t29690: f64, t29798: f64, t29822: f64, t3598: f64, t4945: f64, t6140: f64, t7283: f64, t7300: f64, t7301: f64, t8002: f64, t8011: f64, t8014: f64, t85642: f64, t94395: f64, t94436: f64, t94476: f64) -> f64 {
    let t109137 = 0.16449340668482264365e-1_f64 * t103261 - 0.54831135561607547884e-2_f64 * t94436 - 0.24674011002723396548e-1_f64 * t7283 * t103363 * t8014 - 18.0_f64 * t4945 * t29798 + 0.24674011002723396548e-1_f64 * t7283 * t6140 * t27382 + 0.54831135561607547884e-2_f64 * t94476 - 3.0_f64 * t103464 * t1761 + 0.24125699647107321069e0_f64 * t29678 * t8011 - 0.82246703342411321825e-2_f64 * t7283 * t7300 * t7301 * t22393 - 0.10966227112321509577e-1_f64 * t27549 * t24601 * t85642 * t109060 + 0.10966227112321509577e-1_f64 * t27549 * t24601 * t27774 * t21510 - 0.10966227112321509577e-1_f64 * t27549 * t27820 * t29690 - 0.43864908449286038307e-1_f64 * t94395 * t29822 + 0.82246703342411321826e-2_f64 * t24589 * t103175 * t8002 + 0.43864908449286038307e-1_f64 * t103286 + 2.0_f64 * t1238 * t3598 * t2154 * t22393;
    t109137
}
