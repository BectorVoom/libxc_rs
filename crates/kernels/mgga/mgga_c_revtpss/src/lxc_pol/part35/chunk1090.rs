//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1090/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1090<F: Float>(t114752: F, t1518: F, t1843: F, t1911: F, t2014: F, t2089: F, t2093: F, t2107: F, t2108: F, t22578: F, t22633: F, t22634: F, t22639: F, t23094: F, t29506: F, t29508: F, t30138: F, t30511: F, t30570: F, t30571: F, t30612: F, t34251: F, t4248: F, t5884: F, t5920: F, t5921: F, t651: F, t6765: F, t6934: F, t7359: F, t7983: F, t7984: F, t8065: F, t8075: F, t8109: F, t8111: F, t86825: F) -> (F,) {
    let t116006 = -6.0 * t651 * t8065 * t5920 + t2093 * t23094 - 6.0 * t22639 * t2089 - 6.0 * t651 * t30511 * t1518 - 3.0 * t29506 * t8111 + t114752 * t2108 - t2014 * t2107 * t86825 + 3.0 * t29506 * t8109 + 3.0 * t30612 * t1911 + 3.0 * t8075 * t6934 - 2.0 * t651 * t2089 * t22633 - 12.0 * t30138 * t7984 - 6.0 * t29508 * t7984 - 6.0 * t651 * t6765 * t7983 - 6.0 * t4248 * t30571 - 6.0 * t651 * t1843 * t30570 - 2.0 * t7359 * t22634 - 6.0 * t34251 * t5921 - 6.0 * t7359 * t22578 - 6.0 * t5884 * t8065;
    (t116006,)
}
