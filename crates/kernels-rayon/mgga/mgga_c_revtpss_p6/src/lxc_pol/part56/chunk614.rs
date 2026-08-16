//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 614/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk614(t33: f64, t1113: f64, t5557: f64, t5560: f64, t580: f64, t162: f64, t5556: f64, t189: f64, t512: f64, t1856: f64, t749: f64, t177: f64, t762: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t5564 = piecewise3(t34, 0.0_f64, 4.0_f64 / 9.0_f64 * t5557 * t1113 - 8.0_f64 / 3.0_f64 * t5560 * t580);
    let t5566 = (t5556 + t5564) * t162;
    let t5567 = t5566 * t189;
    let t5568 = t512 * t5567;
    let t5569 = t1856 * t749;
    let t5570 = t512 * t5569;
    let t5571 = t1856 * t177;
    let t5572 = t5571 * t762;
    (t5566, t5568, t5570, t5572)
}
