//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1078/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1078(t11231: f64, t4801: f64, t1042: f64, t1031: f64) -> (f64, f64, f64, f64) {
    let t11232 = t4801 * t11231;
    let t11233 = t1042 * t11232;
    let t11238 = t1031 * t1031;
    let t11239 = 1.0_f64 / t11238;
    (t11232, t11233, t11238, t11239)
}
