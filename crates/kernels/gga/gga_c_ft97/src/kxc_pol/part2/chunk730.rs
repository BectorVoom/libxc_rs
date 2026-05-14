//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 730/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk730<F: Float>(t2157: F, t3578: F, t144: F, t11593: F, t13000: F, t13004: F, t13007: F, t13010: F, t13014: F, t13018: F, t13023: F, t1901: F, t446: F, t9270: F, t9272: F, t9274: F, t9282: F, t9298: F, t9300: F, t9302: F) -> (F, F) {
    let t13030 = t3578 * t2157;
    let t13031 = t144 * t13030;
    let t13037 = 8.0 / 9.0 * t11593 * t13000 - 8.0 / 27.0 * t11593 * t13004 + 2.0 / 9.0 * t1901 * t13007 - 2.0 / 3.0 * t446 * t13010 + t1901 * t13014 / 9.0 + 2.0 / 27.0 * t1901 * t13018 + t1901 * t13023 / 9.0 - 8.0 / 27.0 * t9270 - 8.0 / 27.0 * t9272 + t9274 / 9.0 - t9282 / 9.0 - t446 * t13031 / 3.0 - 8.0 / 81.0 * t9298 - 2.0 / 9.0 * t9300 + 2.0 / 81.0 * t9302;
    (t13030, t13037)
}
