//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 829/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk829(t1045: f64, t12705: f64, t3236: f64, t3248: f64, t3132: f64, t3263: f64, t12697: f64, t205: f64, t12699: f64, t207: f64, t1050: f64, t3139: f64) -> (f64, f64, f64, f64, f64) {
    let t12706 = t12705 * t1045;
    let t12708 = t3236 * t3248;
    let t12710 = t3132 * t3263;
    let t12712 = t205 * t12697;
    let t12713 = t207 * t12699;
    let t12714 = t12712 * t12713;
    let t12716 = t1050 * t3139;
    (t12706, t12708, t12710, t12714, t12716)
}
