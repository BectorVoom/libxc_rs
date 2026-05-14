//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 955/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk955<F: Float>(t16963: F, t1901: F, t2221: F, t4454: F, t4462: F, t50781: F, t64231: F, t64255: F, t64279: F, t77644: F, t77678: F, t77719: F, t77721: F, t77752: F, t9115: F, t13153: F, t144: F, t167: F, t17198: F, t20684: F, t2210: F, t446: F, t569: F, t574: F, t77821: F, t77823: F, t77868: F, t78438: F, t78565: F, t78573: F, t85538: F, t86868: F, t87097: F) -> (F, F) {
    let t87754 = -8.0 / 9.0 * t77644 + 112.0 / 243.0 * t50781 + 4.0 / 3.0 * t77678 + 2.0 / 3.0 * t1901 * t2221 * t16963 * t4462 + 4.0 / 9.0 * t1901 * t9115 * t16963 * t4454 + 8.0 / 9.0 * t64231 + 8.0 / 27.0 * t77719 + 4.0 / 9.0 * t77721 - 4.0 / 9.0 * t77752 - 16.0 / 9.0 * t64255 + 16.0 / 9.0 * t64279;
    let t87780 = -t446 * t574 * t167 * t86868 / 3.0 - 4.0 / 9.0 * t77821 - 8.0 / 9.0 * t77823 + 8.0 / 3.0 * t446 * t569 * t167 * t85538 + 4.0 / 3.0 * t77868 + 2.0 / 3.0 * t1901 * t2210 * t17198 * t4462 + 4.0 * t446 * t144 * t87097 + 4.0 / 3.0 * t1901 * t13153 * t20684 - 8.0 / 9.0 * t78438 + 8.0 / 9.0 * t78565 + 4.0 / 3.0 * t78573;
    (t87754, t87780)
}
