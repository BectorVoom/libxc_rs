//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 522/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk522(t4277: f64, t577: f64, t585: f64, t3733: f64, t1548: f64, t1543: f64, t1552: f64, t4122: f64, t4124: f64, t584: f64, t583: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4278 = t4277 * t577;
    let t4279 = t4278 * t585;
    let t4281 = t3733 * t577;
    let t4282 = t4281 * t1548;
    let t4284 = t1543 * t1552;
    let t4286 = t4122 * t577;
    let t4287 = t584 * t4124;
    let t4288 = t583 * t4287;
    let t4289 = t4286 * t4288;
    let t4291 = t577 * t582;
    (t4278, t4279, t4281, t4282, t4284, t4286, t4287, t4288, t4289, t4291)
}
