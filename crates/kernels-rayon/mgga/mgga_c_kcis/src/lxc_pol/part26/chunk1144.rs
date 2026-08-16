//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1144/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1144(t29254: f64, t446: f64, t27453: f64, t6281: f64, t5709: f64, t28499: f64, t8164: f64, t1394: f64, t4163: f64, t6284: f64, t7923: f64, t5662: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29255 = t446 * t29254;
    let t29256 = t29255 / 8.0_f64;
    let t29258 = t27453 * t6281;
    let t29259 = t5709 * t29258;
    let t29266 = t28499 * t8164;
    let t29267 = t1394 * t29266;
    let t29269 = t4163 * t6284;
    let t29270 = t7923 * t29269;
    let t29271 = t1394 * t29270;
    let t29273 = t5662 * t6281;
    (t29256, t29258, t29259, t29266, t29267, t29269, t29270, t29271, t29273)
}
