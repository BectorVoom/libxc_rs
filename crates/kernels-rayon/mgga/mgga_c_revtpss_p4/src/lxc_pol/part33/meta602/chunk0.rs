//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2026/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2026(t45972: f64, t7565: f64, t45963: f64, t10309: f64, t26754: f64, t12627: f64, t2142: f64, t12640: f64, t11239: f64, t1269: f64, t1276: f64, t2148: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96804 = t45972 * t7565;
    let t96824 = t45963 * t7565;
    let t96827 = t10309 * t26754;
    let t96861 = t12627 * t2142;
    let t96866 = t12640 * t2142;
    let t96881 = t1269 * t11239;
    let t96883 = t2148 * t96881 * t1276;
    (t96804, t96824, t96827, t96861, t96866, t96883)
}
