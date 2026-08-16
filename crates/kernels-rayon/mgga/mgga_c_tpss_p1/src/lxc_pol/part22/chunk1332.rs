//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1332/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1332(t4425: f64, t60738: f64, t1630: f64, t60730: f64, t18436: f64, t4409: f64, t12996: f64, t5716: f64, t12978: f64, t18454: f64, t12982: f64, t12986: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65561 = t60738 * t4425;
    let t65567 = t60730 * t1630;
    let t65570 = t18436 * t4409;
    let t65572 = t5716 * t12996;
    let t65574 = t18454 * t12978;
    let t65576 = t18454 * t12982;
    let t65578 = t18454 * t12986;
    (t65561, t65567, t65570, t65572, t65574, t65576, t65578)
}
