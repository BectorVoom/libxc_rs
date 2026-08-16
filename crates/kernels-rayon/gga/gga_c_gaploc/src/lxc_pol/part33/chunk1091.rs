//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1091/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1091(t10018: f64, t7375: f64, t10007: f64, t1835: f64, t2615: f64, t9438: f64, t124: f64, t15478: f64, t3307: f64, t813: f64, t10013: f64, t2464: f64, t2684: f64) -> (f64, f64, f64, f64, f64) {
    let t28156 = t7375 * t10018;
    let t28160 = t2615 * t9438 * t10007 * t1835;
    let t28229 = t15478 * t124;
    let t28231 = t813 * t28229 * t3307;
    let t28242 = t2684 * t2464 * t10013;
    (t28156, t28160, t28229, t28231, t28242)
}
