//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1166/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1166(t7589: f64, t92187: f64, t2410: f64, t700: f64, t706: f64, t92184: f64, t7580: f64, t26602: f64, t26623: f64, t2389: f64, t26620: f64, t705: f64) -> (f64, f64, f64, f64, f64) {
    let t92188 = t7589 * t92187;
    let t92193 = t7589 * t92184 * t706 * t700 * t2410;
    let t92195 = t7580 * t92187;
    let t92197 = t26602 * t26623;
    let t92201 = t26620 * t2389 * t2410 * t705;
    (t92188, t92193, t92195, t92197, t92201)
}
