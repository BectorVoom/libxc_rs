//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 815/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk815(t26894: f64, t8945: f64, t11239: f64, t487: f64, t1269: f64, t3140: f64, t1276: f64, t2148: f64, t1243: f64, t8939: f64, t2149: f64, t7627: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26895 = t26894 * t8945;
    let t26904 = t487 * t11239;
    let t26916 = t1269 * t3140;
    let t26918 = t2148 * t26916 * t1276;
    let t26921 = t8939 * t1243;
    let t26922 = t2149 * t26921;
    let t26931 = t1243 * t7627;
    (t26895, t26904, t26916, t26918, t26922, t26931)
}
