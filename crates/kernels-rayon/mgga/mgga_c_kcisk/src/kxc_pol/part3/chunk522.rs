//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 522/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk522(t3786: f64, t499: f64, t498: f64, t4235: f64, t1284: f64, t3777: f64, t487: f64, t486: f64, t196: f64, t3729: f64, t306: f64, t476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4236 = t499 * t3786;
    let t4237 = t498 * t4236;
    let t4238 = t4235 * t4237;
    let t4240 = t1284 * t3777;
    let t4241 = t487 * t4240;
    let t4242 = t486 * t4241;
    let t4244 = t3729 * t196;
    let t4253 = t476 * t306;
    (t4236, t4237, t4238, t4240, t4241, t4242, t4244, t4253)
}
