//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1106/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1106(t25610: f64, t8521: f64, t995: f64, t1976: f64, t3057: f64, t11239: f64, t378: f64, t1078: f64, t1982: f64, t3143: f64, t11199: f64, t1981: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25611 = t25610 * t8521;
    let t25629 = t995 * t8521;
    let t25651 = t3057 * t1976;
    let t25669 = t378 * t11239;
    let t25671 = t1982 * t25669 * t1078;
    let t25672 = t3143 * t1976;
    let t25698 = t1981 * t11199;
    (t25611, t25629, t25651, t25671, t25672, t25698)
}
