//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1924/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1924(t27418: f64, t7150: f64, t1976: f64, t4742: f64, t7145: f64, t1695: f64, t7135: f64) -> (f64, f64, f64, f64) {
    let t27419 = t7150 * t27418;
    let t27422 = t1976 * t4742;
    let t27423 = t7145 * t27422;
    let t27426 = t7135 * t1695;
    (t27419, t27422, t27423, t27426)
}
