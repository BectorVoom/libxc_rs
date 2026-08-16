//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 733/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk733(t2623: f64, t815: f64, t2588: f64, t2597: f64, t2526: f64, t823: f64, t755: f64, t774: f64, t7624: f64, t808: f64, t2484: f64, t2615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9026 = t815 * t2623;
    let t9028 = t2588 * t2597;
    let t9030 = t823 * t2526;
    let t9031 = t755 * t9030;
    let t9033 = t2623 * t774;
    let t9034 = t755 * t9033;
    let t9036 = t808 * t7624;
    let t9038 = t2484 * t2615;
    (t9026, t9028, t9031, t9034, t9036, t9038)
}
