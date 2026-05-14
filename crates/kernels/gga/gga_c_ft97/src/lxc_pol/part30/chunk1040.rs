//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1040/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1040<F: Float>(t299: F, t36276: F, t5: F, t113: F, t1275: F, t144289: F, t152530: F, t152574: F, t152615: F, t152644: F, t153492: F, t153520: F, t153548: F, t153674: F, t153712: F, t154908: F, t154945: F, t154983: F, t155009: F, t155028: F, t155066: F, t155092: F, t332: F, t34338: F, t34341: F, t36277: F, t4377: F, t4382: F, t4385: F, t4391: F, t4395: F, t505: F, t911: F, t992: F) -> (F,) {
    let t300 = 10000000.0 <= t299;
    let t155101 = t5 * t36276;
    let t155123 = piecewise3(t300, 0.0, t5 * (t152530 + t152574 + t152615 + t152644 + t153492 + t153520 + t153548 + t153674 + t153712 + t154908 + t154945 + t154983 + t155009 + t155028 + t155066 + t155092) * t332 * t113 / 4.0 + t155101 * t911 / 4.0 + t5 * t36277 * t505 / 4.0 + t144289 * t1275 / 4.0 + t34341 * t4377 / 4.0 + t34341 * t4382 / 4.0 + t34341 * t4385 / 4.0 + t5 * t34338 * t992 / 4.0 + t34341 * t4391 / 4.0 - t34341 * t4395 / 2.0);
    (t155123,)
}
