//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 607/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk607(t1825: f64, t6987: f64, t553: f64, t7722: f64, t1336: f64, t1814: f64, t2013: f64, t544: f64, t6967: f64, t6975: f64, t7734: f64, t7738: f64, t7742: f64) -> (f64, f64, f64) {
    let t7745 = t6987 * t1825;
    let t7747 = t553 * t7722;
    let t7749 = -t6967 - 0.16449340668482264365e-1_f64 * t7734 - t6975 - 0.82246703342411321825e-2_f64 * t7738 + 0.82246703342411321825e-2_f64 * t7742 + t1814 * t2013 - t1336 * t7745 + t544 * t7747;
    (t7745, t7747, t7749)
}
