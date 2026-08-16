//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3332/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3332(t2: f64, t4560: f64, t580: f64, t1587: f64, t18890: f64, t22: f64, t4595: f64, t52505: f64, t4636: f64, t52219: f64, t15101: f64, t15380: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63202 = 4.0_f64 * t4560 * t2 * t580;
    let t63204 = 2.0_f64 * t1587 * t580;
    let t63206 = 6.0_f64 * t18890 * t22;
    let t63212 = 8.0_f64 * t52505 * t4595;
    let t63214 = 0.64327917994770140268e2_f64 * t52219 * t4636;
    let t63216 = 8.0_f64 * t15101 * t15380;
    (t63202, t63204, t63206, t63212, t63214, t63216)
}
