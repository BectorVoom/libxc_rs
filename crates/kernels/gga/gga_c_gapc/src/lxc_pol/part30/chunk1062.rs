//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1062/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1062<F: Float>(t11217: F, t8510: F, t1492: F, t3640: F, t101: F, t11270: F, t190: F, t25047: F, t4049: F, t11207: F, t11211: F, t25176: F, t11208: F, t11210: F, t5248: F, t102: F, t125: F) -> (F, F, F, F, F, F, F) {
    let t35451 = t8510 * t11217;
    let t35453 = t1492 * t3640;
    let t35455 = t11270 * t101;
    let t35458 = t35455 * t4049 * t190 * t25047;
    let t35463 = t25176 * t11207 * t11211;
    let t35466 = t11208 * t11210 * t5248;
    let t35469 = t102 * t125 * t190;
    (t35451, t35453, t35455, t35458, t35463, t35466, t35469)
}
