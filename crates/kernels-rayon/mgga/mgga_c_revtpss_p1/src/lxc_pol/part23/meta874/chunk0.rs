//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2776/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2776(t22126: f64, t2689: f64, t22130: f64, t22081: f64, t9962: f64, t22276: f64, t3989: f64, t22281: f64, t22056: f64, t9765: f64, t22021: f64, t808: f64, t9845: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t74491 = t2689 * t22126;
    let t74493 = t2689 * t22130;
    let t74498 = t9962 * t22081;
    let t74505 = t3989 * t22276;
    let t74507 = t3989 * t22281;
    let t74511 = t9765 * t22056;
    let t74522 = t9845 * t808 * t22021;
    (t74491, t74493, t74498, t74505, t74507, t74511, t74522)
}
