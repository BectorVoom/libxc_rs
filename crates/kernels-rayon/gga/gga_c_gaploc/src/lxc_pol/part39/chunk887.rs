//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 887/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk887(t12691: f64, t2464: f64, t825: f64, t12663: f64, t549: f64, t6111: f64, t12704: f64, t2684: f64, t2628: f64, t9817: f64, t10037: f64, t22256: f64) -> (f64, f64, f64, f64, f64) {
    let t41060 = t825 * t2464 * t12691;
    let t41068 = t6111 * t549 * t12663;
    let t41071 = t2684 * t2464 * t12704;
    let t41075 = t9817 * t2628;
    let t41083 = t10037 * t22256;
    (t41060, t41068, t41071, t41075, t41083)
}
