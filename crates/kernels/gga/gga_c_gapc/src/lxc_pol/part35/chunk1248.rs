//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1248/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1248<F: Float>(t25514: F, t2920: F, t2974: F, t8290: F, t3646: F, t8493: F, t11217: F, t8510: F, t1492: F, t3640: F, t101: F, t11270: F) -> (F, F, F, F, F) {
    let t35447 = t2920 * t25514 * t2974 * t8290;
    let t35449 = t8493 * t3646;
    let t35451 = t8510 * t11217;
    let t35453 = t1492 * t3640;
    let t35455 = t11270 * t101;
    (t35447, t35449, t35451, t35453, t35455)
}
