//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 987/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk987(t334: f64, t369: f64, t86: f64, t1143: f64, t245: f64, t238: f64, t3419: f64, t3393: f64, t3416: f64, t3402: f64, t1157: f64, t752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10541 = 0.11791604938271604938e-1_f64 * t86 * t334 * t369;
    let t10544 = t1143 * t245;
    let t10548 = t86 * t238 * t3419;
    let t10552 = t3393 * t3416;
    let t10554 = t3393 * t3402;
    let t10556 = t752 * t1157;
    (t10541, t10544, t10548, t10552, t10554, t10556)
}
