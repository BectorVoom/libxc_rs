//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1071/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1071(t32399: f64, t32612: f64, t32638: f64, t32741: f64, t3: f64, t2042: f64, t7547: f64, t2113: f64, t7331: f64, t7334: f64, t1459: f64, t8731: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32743 = 2.0_f64 * t32399 + t32612 + t32638 + t32741;
    let t32744 = t3 * t32743;
    let t32755 = param_d * t32743;
    let t32760 = 3.0_f64 * t7547 * t2042;
    let t32762 = 6.0_f64 * t2113 * t7331;
    let t32764 = 3.0_f64 * t2113 * t7334;
    let t32772 = 6.0_f64 * t1459 * t8731;
    (t32743, t32744, t32755, t32760, t32762, t32764, t32772)
}
