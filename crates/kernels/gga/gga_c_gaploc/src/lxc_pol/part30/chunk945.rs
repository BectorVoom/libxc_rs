//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 945/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk945<F: Float>(t10808: F, t11141: F, t224: F, t3513: F, t856: F, t1531: F, t2876: F, t123: F, t3338: F, t2097: F, t3039: F, t3431: F, t5558: F, t744: F, t2012: F, t5639: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11142 = t10808 + t11141;
    let t11143 = t224 * t11142;
    let t12333 = t856 * t3513;
    let t12881 = t2876 * t1531;
    let t12963 = t3338 * t123;
    let t13045 = t3039 * t2097;
    let t13063 = t3431 * t123;
    let t14537 = t744 * t5558;
    let t14549 = t2012 * t5639;
    (t11142, t11143, t12333, t12881, t12963, t13045, t13063, t14537, t14549)
}
