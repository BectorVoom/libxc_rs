//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1071/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1071(t1928: f64, t610: f64, t990: f64, t4426: f64, t6141: f64, t25: f64, t494: f64, t6178: f64, t1599: f64, t1369: f64, t2470: f64, t6164: f64) -> (f64, f64, f64, f64) {
    let t18192 = t610 * t1928 * t990;
    let t18205 = t6141 * t4426 / 324.0_f64;
    let t18210 = t25 * t494;
    let t18211 = t18210 * t6178;
    let t18213 = t1599 * t18211 / 144.0_f64;
    let t18221 = t2470 * t1369;
    let t18222 = t18221 * t6164;
    (t18192, t18205, t18213, t18222)
}
