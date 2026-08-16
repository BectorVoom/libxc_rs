//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2042/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2042(t10416: f64, t13435: f64, t13521: f64, t13648: f64, t1518: f64, t18227: f64, t2014: f64, t2056: f64, t2107: f64, t2322: f64, t25082: f64, t26154: f64, t26674: f64, t26679: f64, t27123: f64, t27126: f64, t27833: f64, t28286: f64, t28588: f64, t28760: f64, t28932: f64, t28935: f64, t49564: f64, t651: f64, t7235: f64, t7359: f64, t7367: f64, t7374: f64, t7536: f64, t7537: f64, t75485: f64, t7732: f64, t7898: f64, t7978: f64, t95088: f64, t97654: f64, t98535: f64) -> f64 {
    let t103999 = -2.0_f64 * t651 * t26674 * t1518 - 2.0_f64 * t75485 * t2056 - 4.0_f64 * t18227 * t7367 - 6.0_f64 * t95088 * t28588 - 2.0_f64 * t7732 * t26154 - 2.0_f64 * t10416 * t7978 - 4.0_f64 * t13435 * t7978 - 4.0_f64 * t2322 * t28760 - 4.0_f64 * t27123 * t7374 - 2.0_f64 * t98535 * t2056 - 4.0_f64 * t27126 * t7367 + 2.0_f64 * t7898 * t26679 + 2.0_f64 * t27833 * t7537 - 2.0_f64 * t7359 * t13521 + 6.0_f64 * t7235 * t28935 + 6.0_f64 * t7235 * t28932 + 12.0_f64 * t25082 * t28286 * t97654 - t2014 * t2107 * t49564 - 2.0_f64 * t2014 * t7536 * t13648;
    t103999
}
