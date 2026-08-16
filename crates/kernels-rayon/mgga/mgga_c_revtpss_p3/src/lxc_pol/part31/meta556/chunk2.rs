//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1965/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1965(t18245: f64, t1937: f64, t30138: f64, t4248: f64, t7735: f64, t1519: f64, t1911: f64, t2011: f64, t28030: f64, t29993: f64, t29998: f64, t30007: f64, t30015: f64, t30113: f64, t30116: f64, t30119: f64, t30125: f64, t30127: f64, t30130: f64, t30150: f64, t569: f64, t5887: f64, t5921: f64, t651: f64, t6934: f64, t6985: f64, t7746: f64, t7894: f64) -> f64 {
    let t30154 = 2.0_f64 * t18245 * t1937;
    let t30156 = 4.0_f64 * t30138 * t1937;
    let t30158 = 4.0_f64 * t4248 * t7735;
    let t30159 = -4.0_f64 * t1519 * t28030 + 2.0_f64 * t1911 * t7894 + t2011 * t6934 - 4.0_f64 * t30116 * t651 - 2.0_f64 * t30119 * t651 + t30150 * t569 - 4.0_f64 * t4248 * t7746 - 4.0_f64 * t5887 * t6985 - 2.0_f64 * t5921 * t6985 - t29993 - t29998 - t30007 + t30015 + t30113 - t30125 - t30127 - t30130 - t30154 - t30156 - t30158;
    t30159
}
