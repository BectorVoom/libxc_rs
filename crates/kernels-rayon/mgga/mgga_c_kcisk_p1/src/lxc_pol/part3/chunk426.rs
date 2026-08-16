//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 426/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk426(t1010: f64, t3274: f64, t224: f64, t1056: f64) -> (f64, f64, f64, f64) {
    let t3275 = t1010 * t3274;
    let t3276 = t224 * t224;
    let t3277 = 1.0_f64 / t3276;
    let t3278 = t1056 * t1056;
    (t3275, t3276, t3277, t3278)
}
