//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 335/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk335(t1137: f64, t793: f64, t1133: f64, t290: f64, t791: f64) -> (f64, f64) {
    let t1138 = t1137 * t793;
    let t1143 = 0.65854491829355115987e0_f64 * t791 * t1138 + 0.65854491829355115987e0_f64 * t290 * t1133;
    (t1138, t1143)
}
