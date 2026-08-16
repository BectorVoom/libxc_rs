//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1000/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1000(t1286: f64, t34357: f64, t376: f64, t108: f64, t34482: f64, t34576: f64, t136044: f64, t136058: f64, t1564: f64, t25990: f64, t25996: f64, t28: f64, t32019: f64, t32396: f64, t3266: f64, t3289: f64, t34589: f64, t379: f64, t38921: f64, t5495: f64, t5501: f64, t5502: f64, t5507: f64, t5748: f64, t6414: f64, t7166: f64, t8411: f64, t942: f64) -> f64 {
    let t144381 = t1286 * t376 * t34357;
    let t144393 = t34482 * t108;
    let t144405 = t1286 * t376 * t34576;
    let t144411 = -t136044 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t5507 * t5748 * t942 + t144381 / 9.0_f64 - t136058 + t6414 * t32396 / 3.0_f64 - 4.0_f64 * t5501 * t38921 * t32019 * t3266 + 2.0_f64 * t5501 * t8411 * t5502 * t25996 - t5501 * t1564 * t144393 * t379 / 18.0_f64 + 2.0_f64 * t5501 * t8411 * t5502 * t25990 + t5495 * t34589 / 3.0_f64 - t144405 / 18.0_f64 - t1286 * t28 * t7166 * t3289 / 3.0_f64;
    t144411
}
