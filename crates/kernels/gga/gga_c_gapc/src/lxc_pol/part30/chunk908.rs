//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 908/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk908<F: Float>(t1087: F, t2299: F, t1908: F, t3140: F, t198: F, t5698: F, t203: F, t19: F, t5700: F, t2981: F, t1649: F, t5391: F, t137: F, t1552: F, t442: F, t5964: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19210 = t1087 * t2299;
    let t19422 = t3140 * t1908;
    let t19507 = t198 * t5698;
    let t19508 = t19507 * t203;
    let t19509 = t5700 * t19;
    let t19510 = t19509 * t2981;
    let t19511 = t19508 * t19510;
    let t19522 = t1649 * t5391;
    let t19530 = t5964 * t1552 * t137 * t442;
    (t19210, t19422, t19507, t19508, t19509, t19510, t19511, t19522, t19530)
}
