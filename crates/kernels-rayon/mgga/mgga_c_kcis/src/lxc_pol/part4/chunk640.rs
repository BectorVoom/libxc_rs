//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 640/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk640(t2844: f64, t421: f64, t2630: f64, t3507: f64, t283: f64, t414: f64, t990: f64) -> (f64, f64, f64) {
    let t3508 = t421 * t2844;
    let t3509 = t3508 * t2630;
    let t3510 = t3507 * t3509;
    let t3513 = t414 * t283;
    let t3514 = t3513 * t990;
    (t3509, t3510, t3514)
}
