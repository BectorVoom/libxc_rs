//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 923/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk923(t7122: f64, t3392: f64, t633: f64, t181: f64, t995: f64, t184: f64, t2800: f64, t2790: f64, t2796: f64, t1627: f64, t3407: f64, t1027: f64, t2722: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10322 = 8.0_f64 / 135.0_f64 * t7122;
    let t10324 = 4.0_f64 / 15.0_f64 * t633 * t3392;
    let t10325 = t995 * t181;
    let t10326 = t10325 * t184;
    let t10328 = 8.0_f64 / 15.0_f64 * t10326 * t2800;
    let t10329 = t2790 * t2796;
    let t10330 = 16.0_f64 / 45.0_f64 * t10329;
    let t10332 = 8.0_f64 / 15.0_f64 * t2790 * t2800;
    let t10334 = 8.0_f64 / 45.0_f64 * t1627 * t3407;
    let t10335 = t1027 * t2722;
    (t10322, t10324, t10328, t10330, t10332, t10334, t10335)
}
