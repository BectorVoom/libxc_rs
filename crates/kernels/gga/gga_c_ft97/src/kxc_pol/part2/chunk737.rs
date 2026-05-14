//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 737/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk737<F: Float>(t3565: F, t604: F, t379: F, t2210: F, t2178: F, t358: F, t2180: F, t920: F, t3052: F, t569: F, t616: F, t2142: F, t3478: F, t574: F, t3483: F, t9276: F) -> (F, F, F, F, F) {
    let t13160 = t604 * t3565;
    let t13161 = t13160 * t379;
    let t13162 = t2210 * t13161;
    let t13165 = t2178 * t358;
    let t13166 = t920 * t2180;
    let t13167 = t13165 * t13166;
    let t13168 = t2210 * t13167;
    let t13173 = t569 * t616 * t3052;
    let t13177 = t574 * t2142 * t3478;
    let t13180 = t9276 * t3483;
    (t13162, t13168, t13173, t13177, t13180)
}
