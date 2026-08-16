//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 749/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk749(t12766: f64, t2343: f64, t2268: f64, t2321: f64, t3371: f64, t882: f64, t10156: f64, t888: f64, t12383: f64, t12386: f64, t12392: f64, t12395: f64, t12397: f64, t12400: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12767 = t2343 * t12766;
    let t12769 = 0.56910013271352299198e-1_f64 * t2268 * t12767;
    let t12770 = t3371 * t2321;
    let t12771 = t882 * t12770;
    let t12773 = t10156 * t888;
    let t12774 = t2268 * t12773;
    let t12782 = -3.0_f64 / 256.0_f64 * t12383 - 27.0_f64 / 8192.0_f64 * t12386 + 27.0_f64 / 524288.0_f64 * t12392 - 9.0_f64 / 524288.0_f64 * t12395 + 9.0_f64 / 8192.0_f64 * t12397 + t12400 / 256.0_f64;
    (t12767, t12769, t12770, t12771, t12773, t12774, t12782)
}
