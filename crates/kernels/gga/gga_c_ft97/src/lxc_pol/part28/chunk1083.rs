//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1083/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1083<F: Float>(t22943: F, t25595: F, t1882: F, t34758: F, t34754: F, t1332: F, t1852: F, t26113: F, t102689: F, t102862: F, t102948: F, t103073: F, t11468: F, t11552: F, t11593: F, t11854: F, t137866: F, t138307: F, t144822: F, t144853: F, t1825: F, t1871: F, t1901: F, t22940: F, t25933: F, t25996: F, t26134: F, t26171: F, t26176: F, t3052: F, t32494: F, t3266: F, t3271: F, t34536: F, t34565: F, t34627: F, t34661: F, t34740: F, t379: F, t432: F, t446: F, t452: F, t5630: F, t6538: F, t8217: F, t83: F, t8557: F) -> (F, F, F) {
    let t146340 = t22943 * t25595;
    let t146358 = t1882 * t34758;
    let t146360 = t1882 * t34754;
    let t146376 = t1852 * t1332 * t26113;
    let t146409 = F::new(4.0) / F::new(3.0) * t446 * t83 * t146340 - F::new(2.0) / F::new(3.0) * t446 * t1871 * t1825 * t34740 - F::new(4.0) / F::new(9.0) * t1901 * t102862 * t26134 - F::new(2.0) / F::new(9.0) * t1901 * t8557 * t34661 * t379 - F::new(4.0) / F::new(9.0) * t1901 * t102689 * t25933 + t146358 / F::new(9.0) + t146360 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t452 * t22940 * t6538 - t446 * t452 * t34536 * t432 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t1901 * t11854 * t34565 * t379 + F::new(2.0) / F::new(9.0) * t137866 + F::new(4.0) / F::new(3.0) * t446 * t83 * t146376 - F::new(2.0) / F::new(9.0) * t1901 * t11468 * t144853 + F::new(2.0) / F::new(27.0) * t1901 * t11552 * t144822 - F::new(4.0) / F::new(3.0) * t1901 * t103073 * t26176 - F::new(4.0) / F::new(9.0) * t11593 * t8217 * t32494 * t3052 + F::new(8.0) * t1901 * t102948 * t32494 * t3266 + F::new(2.0) * t1901 * t26171 * t138307 * t3271 - F::new(4.0) * t1901 * t26171 * t5630 * t25996 + t446 * t452 * t1825 * t34627 / F::new(3.0);
    (t146340, t146376, t146409)
}
