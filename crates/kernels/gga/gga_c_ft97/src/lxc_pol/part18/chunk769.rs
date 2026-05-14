//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 769/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk769<F: Float>(t13160: F, t379: F, t2210: F, t2178: F, t358: F, t2180: F, t920: F, t3052: F, t569: F, t616: F, t2142: F, t3478: F, t574: F, t3483: F, t9276: F, t144: F) -> (F, F, F, F, F, F, F, F) {
    let t13161 = t13160 * t379;
    let t13162 = t2210 * t13161;
    let t13165 = t2178 * t358;
    let t13166 = t920 * t2180;
    let t13167 = t13165 * t13166;
    let t13168 = t2210 * t13167;
    let t13173 = t569 * t616 * t3052;
    let t13177 = t574 * t2142 * t3478;
    let t13180 = t9276 * t3483;
    let t13181 = t144 * t13180;
    (t13161, t13162, t13166, t13167, t13168, t13173, t13177, t13181)
}
