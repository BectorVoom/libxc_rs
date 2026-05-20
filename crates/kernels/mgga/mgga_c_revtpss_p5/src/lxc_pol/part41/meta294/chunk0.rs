//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1058/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1058<F: Float>(t2967: F, t941: F, t2966: F, t307: F, t302: F, t11132: F, t11337: F, t944: F, t2969: F, t310: F, t3011: F, t960: F) -> (F, F, F, F, F, F, F) {
    let t11404 = t941 * t2967;
    let t11408 = F::new(1.0) / t2966 / t307;
    let t11409 = t302 * t11408;
    let t11422 = F::cast_from(0.16068111111111111111e1_f64) * t11132;
    let t11423 = F::cast_from(0.46308888888888888888e0_f64) * t11337;
    let t11449 = F::new(1.0) / t2966 / t944;
    let t11450 = t302 * t11449;
    let t11452 = F::new(1.0) / t2969 / t310;
    let t11461 = t960 * t3011;
    (t11404, t11409, t11422, t11423, t11450, t11452, t11461)
}
