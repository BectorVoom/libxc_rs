//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1393/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1393(t11981: f64, t1391: f64, t1392: f64, t2487: f64, t34631: f64, t34634: f64, t34636: f64, t34638: f64, t34640: f64, t34643: f64, t34645: f64, t34648: f64, t34650: f64, t34652: f64, t34659: f64, t34662: f64, t34665: f64, t34668: f64) -> f64 {
    let t38663 = t34631 + t34634 + 0.11360866949309851756e0_f64 * t2487 * t1391 * t1392 * t11981 + t34636 + t34638 - t34640 - t34643 - t34645 + t34648 + t34650 + t34652 - t34659 - t34662 + t34665 + t34668;
    t38663
}
