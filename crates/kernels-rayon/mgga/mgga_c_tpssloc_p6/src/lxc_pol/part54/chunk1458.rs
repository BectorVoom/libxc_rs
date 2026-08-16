//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1458/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1458(t671: f64, t8828: f64, t104977: f64, t117533: f64, t120876: f64, t120877: f64, t120878: f64, t120881: f64, t120885: f64, t1459: f64, t19456: f64, t2040: f64, t27145: f64, t27863: f64, t32318: f64, t32350: f64, t4028: f64, t4037: f64, t7050: f64, t7408: f64, t7787: f64, t8690: f64, t8835: f64) -> (f64, f64) {
    let t124715 = t8828 * t671;
    let t124726 = -2.0_f64 * t104977 * t2040 - 2.0_f64 * t117533 * t1459 - 2.0_f64 * t124715 * t1459 - 2.0_f64 * t19456 * t8835 + t27145 * t8690 - 2.0_f64 * t27863 * t7050 - 2.0_f64 * t32318 * t4028 - 2.0_f64 * t32350 * t4037 - t7408 * t7787 + t120876 - t120877 - t120878 - t120881 + t120885;
    (t124715, t124726)
}
