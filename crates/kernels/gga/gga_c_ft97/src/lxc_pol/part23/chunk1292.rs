//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1292/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1292<F: Float>(t31190: F, t8392: F, t1449: F, t18386: F, t2568: F, t5064: F, t97299: F, t1882: F, t31283: F, t111512: F, t111523: F, t111528: F, t111530: F, t1131: F, t13839: F, t14127: F, t14159: F, t18412: F, t18646: F, t1901: F, t242: F, t24668: F, t27889: F, t28124: F, t28308: F, t28364: F, t446: F, t53797: F, t65408: F, t729: F, t762: F, t98078: F, t98123: F) -> (F, F, F) {
    let t125058 = t8392 * t31190;
    let t125061 = t2568 * t1449 * t18386;
    let t125072 = t97299 * t5064;
    let t125076 = t1882 * t31283;
    let t125087 = -4.0 / 27.0 * t98078 - 8.0 / 27.0 * t111512 + 2.0 / 3.0 * t446 * t729 * t762 * t27889 * t1131 + t111523 - 2.0 / 27.0 * t125058 + 2.0 / 3.0 * t446 * t242 * t125061 + 2.0 / 9.0 * t1901 * t13839 * t28308 - 2.0 / 3.0 * t1901 * t14127 * t24668 * t18646 - t111528 + 2.0 / 3.0 * t446 * t242 * t125072 + 2.0 / 9.0 * t125076 + t111530 + 4.0 / 9.0 * t53797 * t98123 * t18412 - 4.0 / 3.0 * t1901 * t65408 * t28364 + 2.0 / 9.0 * t1901 * t14159 * t28124;
    (t125061, t125072, t125087)
}
