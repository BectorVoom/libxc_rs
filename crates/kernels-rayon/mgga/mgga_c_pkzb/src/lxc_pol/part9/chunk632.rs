//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 632/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk632(t2819: f64, t702: f64, t1833: f64, t1944: f64, t2730: f64, t2741: f64) -> (f64, f64) {
    let t2820 = t2819 * t702;
    let t2826 = t1944 - 0.92708333333333333333e-2_f64 * t1833 - 0.92708333333333333333e-2_f64 * t2730 + 0.278125e-1_f64 * t2741;
    (t2820, t2826)
}
