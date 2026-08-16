//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1231/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1231(t114752: f64, t1518: f64, t1843: f64, t1911: f64, t2014: f64, t2089: f64, t2093: f64, t2107: f64, t2108: f64, t22578: f64, t22633: f64, t22634: f64, t22639: f64, t23094: f64, t29506: f64, t29508: f64, t30138: f64, t30511: f64, t30570: f64, t30571: f64, t30612: f64, t34251: f64, t4248: f64, t5884: f64, t5920: f64, t5921: f64, t651: f64, t6765: f64, t6934: f64, t7359: f64, t7983: f64, t7984: f64, t8065: f64, t8075: f64, t8109: f64, t8111: f64, t86825: f64) -> f64 {
    let t116006 = -6.0_f64 * t651 * t8065 * t5920 + t2093 * t23094 - 6.0_f64 * t22639 * t2089 - 6.0_f64 * t651 * t30511 * t1518 - 3.0_f64 * t29506 * t8111 + t114752 * t2108 - t2014 * t2107 * t86825 + 3.0_f64 * t29506 * t8109 + 3.0_f64 * t30612 * t1911 + 3.0_f64 * t8075 * t6934 - 2.0_f64 * t651 * t2089 * t22633 - 12.0_f64 * t30138 * t7984 - 6.0_f64 * t29508 * t7984 - 6.0_f64 * t651 * t6765 * t7983 - 6.0_f64 * t4248 * t30571 - 6.0_f64 * t651 * t1843 * t30570 - 2.0_f64 * t7359 * t22634 - 6.0_f64 * t34251 * t5921 - 6.0_f64 * t7359 * t22578 - 6.0_f64 * t5884 * t8065;
    t116006
}
