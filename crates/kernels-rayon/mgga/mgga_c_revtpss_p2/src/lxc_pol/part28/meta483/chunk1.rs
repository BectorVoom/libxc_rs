//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1835/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1835(t1976: f64, t989: f64, t225: f64, t25586: f64, t385: f64, t11239: f64, t378: f64, t1078: f64, t1982: f64, t3143: f64, t3151: f64, t3304: f64) -> (f64, f64, f64, f64) {
    let t25658 = t989 * t1976;
    let t25662 = t25586 * t225 * t385;
    let t25669 = t378 * t11239;
    let t25671 = t1982 * t25669 * t1078;
    let t25672 = t3143 * t1976;
    let t25674 = t25672 * t3151 * t3304;
    (t25658, t25662, t25671, t25674)
}
