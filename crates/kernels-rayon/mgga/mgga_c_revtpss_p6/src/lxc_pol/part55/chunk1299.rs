//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1299/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1299(t1921: f64, t8900: f64, t1913: f64, t8909: f64, t124411: f64, t124413: f64, t124418: f64, t124420: f64, t124431: f64, t131119: f64, t1914: f64, t2111: f64, t2118: f64, t2168: f64, t2172: f64, t28945: f64, t28993: f64, t29469: f64, t29490: f64, t3: f64, t33338: f64, t575: f64, t5790: f64, t7691: f64, t7700: f64, t8114: f64, t8130: f64) -> f64 {
    let t131123 = t8900 * t1921;
    let t131128 = t1913 * t8909;
    let t131131 = t131119 * t3 * t575 + t1914 * t33338 + t2111 * t29490 + t2118 * t29469 + t2168 * t28993 + t2172 * t28945 + t5790 * t8909 + t7691 * t8130 + t7700 * t8114 + t124411 + t124413 + t124418 + t124420 + t124431 + t131123 + t131128;
    t131131
}
