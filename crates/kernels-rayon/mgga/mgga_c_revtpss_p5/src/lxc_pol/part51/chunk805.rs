//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 805/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk805(t25698: f64, t7143: f64, t3336: f64, t7177: f64, t11108: f64, t1989: f64, t2411: f64, t33: f64) -> (f64, f64, f64, f64) {
    let t25699 = t25698 * t7143;
    let t25709 = t7177 * t3336;
    let t25713 = t1989 * t11108;
    let t25759 = t2411 * t33;
    (t25699, t25709, t25713, t25759)
}
