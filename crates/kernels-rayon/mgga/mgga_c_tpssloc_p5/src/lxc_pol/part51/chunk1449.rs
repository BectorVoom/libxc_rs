//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1449/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1449(t115630: f64, t120628: f64, t120629: f64, t120633: f64, t122399: f64, t122406: f64, t122439: f64, t122470: f64, t122495: f64, t122515: f64, t122542: f64, t1375: f64, t1378: f64, t27132: f64, t33301: f64, t33316: f64, t33320: f64, t3758: f64, t3882: f64, t539: f64, t568: f64, t6958: f64) -> f64 {
    let t122547 = -0.49348022005446793095e-1_f64 * t122399 + t115630 + 2.0_f64 * t3758 * t33316 + 2.0_f64 * t3882 * t33316 - 0.82246703342411321825e-2_f64 * t122406 + t120628 + t539 * t122439 * t568 + 2.0_f64 * t6958 * t27132 + 2.0_f64 * t3882 * t33320 + 2.0_f64 * t3882 * t33301 + t120629 - t1375 * t1378 * (t122470 + t122495 + t122515 + t122542) + t120633;
    t122547
}
