//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1241/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1241(t10841: f64, t25245: f64, t10867: f64, t64: f64, t239: f64, t820: f64, t10874: f64, t2681: f64, t7043: f64, t857: f64, t25222: f64, t2656: f64) -> (f64, f64, f64, f64) {
    let t93058 = t25245 * t10841;
    let t93060 = t10867 * t64;
    let t93062 = t820 * t93060 * t239;
    let t93063 = t93062 * t10874;
    let t93066 = t820 * t7043 * t2681;
    let t93067 = t93066 * t857;
    let t93069 = t25222 * t2656;
    (t93058, t93063, t93067, t93069)
}
