//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 828/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk828(t1235: f64, t5721: f64, t1239: f64, t1765: f64, t522: f64, t64: f64, t234: f64, t339: f64) -> (f64, f64, f64, f64) {
    let t5722 = t5721 * t1235;
    let t5724 = t1765 * t1239;
    let t5726 = t522 * t64;
    let t5728 = t339 * t5726 * t234;
    (t5722, t5724, t5726, t5728)
}
