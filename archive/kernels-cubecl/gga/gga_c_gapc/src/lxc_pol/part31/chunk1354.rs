//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1354/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1354<F: Float>(t13281: F, t1617: F, t3808: F, t2967: F, t31767: F, t2822: F, t3832: F, t7063: F, t10529: F, t8613: F, t24915: F, t3568: F) -> (F, F, F, F, F) {
    let t36288 = F::cast_from(24.0_f64) * t13281 * t3808 * t1617;
    let t36290 = F::cast_from(4.0_f64) * t31767 * t2967;
    let t36293 = F::cast_from(6.0_f64) * t7063 * t3832 * t2822;
    let t36295 = F::cast_from(4.0_f64) * t10529 * t8613;
    let t36297 = F::cast_from(4.0_f64) * t24915 * t3568;
    (t36288, t36290, t36293, t36295, t36297)
}
