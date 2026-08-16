//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 693/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk693(t1470: f64, t603: f64, t1493: f64, t76: f64, t1937: f64, t4248: f64, t1518: f64, t94: f64) -> (f64, f64, f64, f64) {
    let t7709 = t603 * t1470;
    let t7719 = t76 * t1493;
    let t7731 = 2.0_f64 * t4248 * t1937;
    let t7732 = t94 * t1518;
    (t7709, t7719, t7731, t7732)
}
