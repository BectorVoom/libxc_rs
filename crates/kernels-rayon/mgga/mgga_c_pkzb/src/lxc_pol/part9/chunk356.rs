//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 356/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk356(t1249: f64, t945: f64, t1245: f64, t397: f64, t943: f64) -> (f64, f64) {
    let t1250 = t1249 * t945;
    let t1255 = 0.65854491829355115987e0_f64 * t943 * t1250 + 0.65854491829355115987e0_f64 * t397 * t1245;
    (t1250, t1255)
}
