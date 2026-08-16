//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 878/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk878(t1375: f64, t2016: f64, t2092: f64, t568: f64, t6958: f64, t7194: f64, t8457: f64, t8461: f64, t8613: f64, t8618: f64, t8623: f64, t8627: f64, t8637: f64) -> f64 {
    let t8639 = t8457 - t8461 + 0.82246703342411321825e-2_f64 * t8613 + t8618 * t568 - t7194 * t2016 - 0.82246703342411321825e-2_f64 * t8623 - t6958 * t2092 + 2.0_f64 * t1375 * t8627 - t1375 * t8637;
    t8639
}
