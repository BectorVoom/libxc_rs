//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1981/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1981(t10115: f64, t555: f64, t4146: f64, t198: f64, t775: f64, t13026: f64, t65: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t21661: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t50080 = t198 * t775;
    let t57549 = t65 * t13026;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60670 = t21661 * t602;
    (t47567, t47672, t50080, t57549, t60221, t60224, t60670)
}
