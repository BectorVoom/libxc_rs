//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1083/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1083(t22943: f64, t25595: f64, t1882: f64, t34758: f64, t34754: f64, t1332: f64, t1852: f64, t26113: f64, t102689: f64, t102862: f64, t102948: f64, t103073: f64, t11468: f64, t11552: f64, t11593: f64, t11854: f64, t137866: f64, t138307: f64, t144822: f64, t144853: f64, t1825: f64, t1871: f64, t1901: f64, t22940: f64, t25933: f64, t25996: f64, t26134: f64, t26171: f64, t26176: f64, t3052: f64, t32494: f64, t3266: f64, t3271: f64, t34536: f64, t34565: f64, t34627: f64, t34661: f64, t34740: f64, t379: f64, t432: f64, t446: f64, t452: f64, t5630: f64, t6538: f64, t8217: f64, t83: f64, t8557: f64) -> (f64, f64, f64) {
    let t146340 = t22943 * t25595;
    let t146358 = t1882 * t34758;
    let t146360 = t1882 * t34754;
    let t146376 = t1852 * t1332 * t26113;
    let t146409 = 4.0_f64 / 3.0_f64 * t446 * t83 * t146340 - 2.0_f64 / 3.0_f64 * t446 * t1871 * t1825 * t34740 - 4.0_f64 / 9.0_f64 * t1901 * t102862 * t26134 - 2.0_f64 / 9.0_f64 * t1901 * t8557 * t34661 * t379 - 4.0_f64 / 9.0_f64 * t1901 * t102689 * t25933 + t146358 / 9.0_f64 + t146360 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t452 * t22940 * t6538 - t446 * t452 * t34536 * t432 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t11854 * t34565 * t379 + 2.0_f64 / 9.0_f64 * t137866 + 4.0_f64 / 3.0_f64 * t446 * t83 * t146376 - 2.0_f64 / 9.0_f64 * t1901 * t11468 * t144853 + 2.0_f64 / 27.0_f64 * t1901 * t11552 * t144822 - 4.0_f64 / 3.0_f64 * t1901 * t103073 * t26176 - 4.0_f64 / 9.0_f64 * t11593 * t8217 * t32494 * t3052 + 8.0_f64 * t1901 * t102948 * t32494 * t3266 + 2.0_f64 * t1901 * t26171 * t138307 * t3271 - 4.0_f64 * t1901 * t26171 * t5630 * t25996 + t446 * t452 * t1825 * t34627 / 3.0_f64;
    (t146340, t146376, t146409)
}
