//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1141/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1141(t1310: f64, t5920: f64, t116: f64, t5876: f64, t4343: f64, t4542: f64, t2404: f64, t5966: f64, t14613: f64, t162: f64, t4403: f64, t14312: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18242 = t1310 * t5920;
    let t18245 = t5876 * t116;
    let t18253 = t4542 * t4343;
    let t18256 = t2404 * t5966;
    let t18259 = t14613 * t162;
    let t18261 = 24.0_f64 * t18259 * t4403;
    let t18262 = 2.0_f64 * t14312;
    (t18242, t18245, t18253, t18256, t18261, t18262)
}
