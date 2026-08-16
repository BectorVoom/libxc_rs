//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1210/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1210(t2533: f64, t26651: f64, t2153: f64, t2538: f64, t9312: f64, t31274: f64, t7612: f64, t26607: f64, t26623: f64, t26620: f64, t700: f64, t9236: f64) -> (f64, f64, f64, f64, f64) {
    let t92165 = 3.0_f64 * t2533 * t26651;
    let t92168 = 2.0_f64 * t2538 * t2153 * t9312;
    let t92170 = 6.0_f64 * t31274 * t7612;
    let t92171 = t26607 * t26623;
    let t92174 = t26620 * t9236 * t700;
    (t92165, t92168, t92170, t92171, t92174)
}
