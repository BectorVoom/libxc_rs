//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1099/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1099(t37637: f64, t39469: f64, t24906: f64, t37943: f64, t37945: f64, t24916: f64, t37949: f64, t37616: f64, t37630: f64, t37634: f64, t37639: f64, t10810: f64, t574: f64, t8066: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39470 = t37637 * t39469;
    let t39482 = t37943 * t37945 * t24906;
    let t39485 = t37949 * t37945 * t24916;
    let t39487 = 0.84755945902752848174e0_f64 * t37616;
    let t39492 = 0.11902492299418487743e0_f64 * t37630;
    let t39493 = 0.35707476898255463229e0_f64 * t37634;
    let t39494 = 0.28914548798370980346e-3_f64 * t37639;
    let t39499 = t574 * t10810 * t8066;
    (t39470, t39482, t39485, t39487, t39492, t39493, t39494, t39499)
}
