//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1348/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1348(t100165: f64, t100254: f64, t100324: f64, t100449: f64, t1539: f64, t1949: f64, t21138: f64, t21458: f64, t23601: f64, t23678: f64, t23696: f64, t25516: f64, t28602: f64, t28670: f64, t4669: f64, t5677: f64, t6687: f64, t6784: f64, t6785: f64, t83245: f64, t89310: f64, t89366: f64, t89473: f64) -> f64 {
    let t106176 = 6.0_f64 * t4669 * t28602 - 0.82246703342411321826e-2_f64 * t100254 - 0.54831135561607547884e-2_f64 * t89310 - 0.82246703342411321825e-2_f64 * t6687 * t21458 * t1949 + 0.16449340668482264365e-1_f64 * t83245 * t100165 * t23678 * t1539 + 0.82246703342411321826e-2_f64 * t6687 * t6784 * t100449 * t1539 + 0.24674011002723396548e-1_f64 * t23601 * t89473 * t28670 + 0.10966227112321509577e-1_f64 * t6687 * t23696 * t25516 * t5677 - 0.18277045187202515961e-2_f64 * t89366 + 0.82246703342411321826e-2_f64 * t100324 + 0.16449340668482264365e-1_f64 * t6687 * t6784 * t6785 * t21138;
    t106176
}
