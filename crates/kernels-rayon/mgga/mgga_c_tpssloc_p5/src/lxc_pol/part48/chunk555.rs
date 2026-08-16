//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 555/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk555(t1912: f64, t259: f64, t2597: f64, t2713: f64, t6549: f64, t6557: f64, t6565: f64, t6569: f64, t6574: f64, t6576: f64, t6625: f64, t6627: f64, t6632: f64, t6663: f64, t855: f64, t866: f64) -> f64 {
    let t6665 = -t6549 - 0.16449340668482264365e-1_f64 * t6557 - t6565 + 0.82246703342411321825e-2_f64 * t6569 - 0.82246703342411321825e-2_f64 * t6574 + t6576 * t259 + t6625 * t259 - t6627 * t866 - t2597 * t1912 - t2713 * t1912 + 2.0_f64 * t855 * t6632 - t855 * t6663;
    t6665
}
