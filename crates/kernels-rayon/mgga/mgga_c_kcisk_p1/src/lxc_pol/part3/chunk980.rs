//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 980/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk980(t4265: f64, t4288: f64, t4274: f64, t1471: f64, t3283: f64, t4277: f64, t12924: f64, t1472: f64, t12868: f64, t6287: f64, t12983: f64, t6279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14444 = t4265 * t4288;
    let t14446 = t4265 * t4274;
    let t14449 = t1471 * t4277 * t3283;
    let t14453 = t1471 * t1472 * t12924;
    let t14458 = t6287 * t12868;
    let t14461 = t6279 * t12983;
    (t14444, t14446, t14449, t14453, t14458, t14461)
}
