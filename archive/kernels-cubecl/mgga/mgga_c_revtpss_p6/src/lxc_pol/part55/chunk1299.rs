//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1299/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1299<F: Float>(t1921: F, t8900: F, t1913: F, t8909: F, t124411: F, t124413: F, t124418: F, t124420: F, t124431: F, t131119: F, t1914: F, t2111: F, t2118: F, t2168: F, t2172: F, t28945: F, t28993: F, t29469: F, t29490: F, t3: F, t33338: F, t575: F, t5790: F, t7691: F, t7700: F, t8114: F, t8130: F) -> F {
    let t131123 = t8900 * t1921;
    let t131128 = t1913 * t8909;
    let t131131 = t131119 * t3 * t575 + t1914 * t33338 + t2111 * t29490 + t2118 * t29469 + t2168 * t28993 + t2172 * t28945 + t5790 * t8909 + t7691 * t8130 + t7700 * t8114 + t124411 + t124413 + t124418 + t124420 + t124431 + t131123 + t131128;
    t131131
}
