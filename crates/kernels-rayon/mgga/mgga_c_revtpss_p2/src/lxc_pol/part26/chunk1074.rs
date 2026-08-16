//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1074/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1074(t26333: f64, t545: f64, t2028: f64, t225: f64, t26079: f64, t26255: f64, t4003: f64, t1444: f64, t7296: f64, t7506: f64, t2097: f64, t4131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26334 = t545 * t26333;
    let t26335 = t2028 * t26334;
    let t26338 = t26333 * t225;
    let t26343 = t26079 * t26255 * t4003;
    let t26347 = t7296 * t7506 * t1444;
    let t26351 = t7296 * t2097 * t4131;
    (t26334, t26335, t26338, t26343, t26347, t26351)
}
