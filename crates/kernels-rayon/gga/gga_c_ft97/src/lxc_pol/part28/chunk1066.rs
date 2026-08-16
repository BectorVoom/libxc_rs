//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1066/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1066(t1286: f64, t136116: f64, t137476: f64, t137497: f64, t144765: f64, t145705: f64, t145719: f64, t145731: f64, t25523: f64, t26128: f64, t28: f64, t32011: f64, t32338: f64, t3266: f64, t34354: f64, t34565: f64, t34568: f64, t492: f64, t5495: f64, t5501: f64, t6421: f64, t8411: f64, t8418: f64) -> f64 {
    let t145733 = t137476 / 9.0_f64 - 2.0_f64 * t144765 - 2.0_f64 * t145705 + t1286 * t28 * t32338 * t25523 + t1286 * t28 * t32338 * t26128 - 24.0_f64 * t8418 * t34565 * t492 - 12.0_f64 * t8418 * t34568 * t492 + 8.0_f64 * t145719 - t137497 / 18.0_f64 + t5501 * t8411 * t32011 * t3266 - t1286 * t28 * t136116 * t6421 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t5495 * t34354 - t145731 / 9.0_f64;
    t145733
}
