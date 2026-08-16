//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1300/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1300(t25260: f64, t787: f64, t9824: f64, t123: f64, t8720: f64, t883: f64, t2684: f64, t2685: f64, t2464: f64, t2465: f64, t8469: f64, t10879: f64, t7416: f64) -> (f64, f64, f64, f64, f64) {
    let t33375 = t787 * t25260 * t9824;
    let t33376 = 0.14896037479937677779e-1_f64 * t33375;
    let t33378 = t8720 * t123 * t883;
    let t33380 = t2684 * t2685 * t33378;
    let t33381 = 0.19171462976960374838e0_f64 * t33380;
    let t33384 = t2684 * t2464 * t2465 * t8469;
    let t33385 = 0.85206502119823888168e-1_f64 * t33384;
    let t33386 = t7416 * t10879;
    (t33376, t33378, t33381, t33385, t33386)
}
