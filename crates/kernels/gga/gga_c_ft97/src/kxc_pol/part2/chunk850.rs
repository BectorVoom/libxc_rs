//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 850/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk850<F: Float>(t848: F, t871: F, t2883: F, t3699: F, t2409: F, t4150: F, t2881: F, t15129: F, t296: F, t319: F, t668: F, t835: F, t1882: F, t4248: F, t4301: F, t15136: F) -> (F, F, F, F, F, F, F) {
    let t15254 = t848 * t871;
    let t15255 = t3699 * t2883;
    let t15256 = t15254 * t15255;
    let t15259 = t4150 * t2409;
    let t15260 = t2881 * t15259;
    let t15263 = t296 * t15129;
    let t15267 = t835 * t319 * t668;
    let t15271 = 2.0 / 9.0 * t1882 * t4248;
    let t15273 = 2.0 / 9.0 * t1882 * t4301;
    let t15274 = t296 * t15136;
    (t15256, t15260, t15263, t15267, t15271, t15273, t15274)
}
