//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1288/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1288(t1630: f64, t60730: f64, t18436: f64, t4409: f64, t18444: f64, t339: f64, t4419: f64, t790: f64, t1246: f64, t136: f64, t1693: f64, t19468: f64, t19470: f64, t5543: f64) -> (f64, f64, f64, f64, f64) {
    let t65567 = t60730 * t1630;
    let t65570 = t18436 * t4409;
    let t65592 = t339 * t18444 * t790 * t4419;
    let t65595 = t1693 * t1246 * t136;
    let t65600 = t5543 * t19468 * t19470;
    (t65567, t65570, t65592, t65595, t65600)
}
