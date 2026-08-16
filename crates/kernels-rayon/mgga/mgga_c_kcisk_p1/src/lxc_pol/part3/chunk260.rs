//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 260/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk260(t411: f64, t338: f64, t157: f64, t301: f64, t342: f64, t341: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1218 = t411 * t411;
    let t1219 = 1.0_f64 / t1218;
    let t1220 = t338 * t1219;
    let t1222 = t342 * t157 * t301;
    let t1223 = 0.17808333333333333333e-1_f64 * t1222;
    let t1224 = t341 * t69;
    (t1218, t1219, t1220, t1222, t1223, t1224)
}
