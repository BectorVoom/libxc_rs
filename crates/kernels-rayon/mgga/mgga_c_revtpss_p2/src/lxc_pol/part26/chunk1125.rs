//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1125/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1125(t2726: f64, t93054: f64, t10841: f64, t25245: f64, t10867: f64, t64: f64, t239: f64, t820: f64, t10874: f64, t2681: f64, t7043: f64, t857: f64) -> (f64, f64, f64, f64) {
    let t93055 = t93054 * t2726;
    let t93058 = t25245 * t10841;
    let t93060 = t10867 * t64;
    let t93062 = t820 * t93060 * t239;
    let t93063 = t93062 * t10874;
    let t93066 = t820 * t7043 * t2681;
    let t93067 = t93066 * t857;
    (t93055, t93058, t93063, t93067)
}
