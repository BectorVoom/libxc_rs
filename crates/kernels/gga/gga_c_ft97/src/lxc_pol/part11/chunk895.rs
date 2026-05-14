//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 895/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk895<F: Float>(t157: F, t40465: F, t8392: F, t9425: F, t24: F, t32905: F, t2159: F, t8232: F, t9094: F, t9129: F, t1882: F, t9109: F, t144: F, t167: F, t1901: F, t2075: F, t2179: F, t2180: F, t2185: F, t2205: F, t2210: F, t3440: F, t379: F, t38064: F, t38930: F, t39658: F, t446: F, t558: F, t569: F, t574: F, t616: F, t7959: F, t9007: F, t9276: F, t9311: F, t9327: F, t9344: F, t9419: F, t9462: F) -> (F, F, F, F, F, F) {
    let t40808 = t40465 * t157;
    let t40828 = t8392 * t9425;
    let t40830 = t24 * t32905;
    let t40835 = t8232 * t2159;
    let t40837 = t8392 * t9094;
    let t40840 = t8392 * t9129;
    let t40847 = t1882 * t9109;
    let t40880 = 8.0 / 9.0 * t40840 + 8.0 / 3.0 * t446 * t2185 * t167 * t9007 * t558 - 8.0 / 9.0 * t40847 - 12.0 * t446 * t144 * t39658 - 4.0 * t446 * t574 * t2179 * t2180 * t2075 - 8.0 * t446 * t574 * t9276 * t9311 + 16.0 / 9.0 * t446 * t2205 * t616 * t7959 + 40.0 / 27.0 * t446 * t9327 * t167 * t38064 - 4.0 / 9.0 * t446 * t569 * t9462 * t379 - 4.0 * t1901 * t2210 * t3440 * t38930 - 8.0 / 3.0 * t1901 * t9419 * t9344;
    (t40808, t40828, t40830, t40835, t40837, t40880)
}
