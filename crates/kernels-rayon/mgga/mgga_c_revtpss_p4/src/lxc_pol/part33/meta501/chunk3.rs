//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1814/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1814(t11239: f64, t487: f64, t1276: f64, t2148: f64, t2142: f64, t3596: f64, t1269: f64, t3140: f64, t1243: f64, t8939: f64) -> (f64, f64, f64, f64) {
    let t26904 = t487 * t11239;
    let t26906 = t2148 * t26904 * t1276;
    let t26907 = t3596 * t2142;
    let t26916 = t1269 * t3140;
    let t26918 = t2148 * t26916 * t1276;
    let t26921 = t8939 * t1243;
    (t26906, t26907, t26918, t26921)
}
