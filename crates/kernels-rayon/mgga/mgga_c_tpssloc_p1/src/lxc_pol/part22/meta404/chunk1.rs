//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1703/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1703(t18211: f64, t4900: f64, t15382: f64, t15390: f64, t1171: f64, t6109: f64, t6011: f64, t699: f64) -> (f64, f64, f64, f64) {
    let t18475 = t4900 * t18211;
    let t18484 = t15390 * t15382;
    let t18489 = t6109 * t1171;
    let t18494 = t699 * t6011;
    (t18475, t18484, t18489, t18494)
}
