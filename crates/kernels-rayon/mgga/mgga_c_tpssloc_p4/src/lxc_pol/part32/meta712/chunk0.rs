//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2233/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2233(t17037: f64, t1888: f64, t22996: f64, t232: f64, t58204: f64, t6646: f64, t2632: f64, t58166: f64, t28423: f64, t6579: f64, t28427: f64, t1902: f64, t5611: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98478 = t1888 * t22996 * t17037;
    let t98482 = t1888 * t6646 * t58204 * t232;
    let t98486 = t1888 * t22996 * t58166 * t2632;
    let t98488 = t6579 * t28423;
    let t98490 = t6579 * t28427;
    let t98494 = t1902 * t5611;
    (t98478, t98482, t98486, t98488, t98490, t98494)
}
