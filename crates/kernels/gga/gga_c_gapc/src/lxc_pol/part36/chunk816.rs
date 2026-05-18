//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 816/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk816<F: Float>(t2639: F, t9760: F, t1936: F, t2520: F, t3345: F, t3424: F, t3427: F, t3431: F, t7675: F, t3437: F, t8915: F, t3440: F) -> (F, F, F, F, F, F) {
    let t9761 = t9760 * t2639;
    let t9763 = t2520 * t1936;
    let t9764 = t9763 * t3345;
    let t9766 = t3424 * t3427;
    let t9768 = t7675 * t3431;
    let t9770 = t3437 * t8915;
    let t9771 = t9770 * t3440;
    (t9761, t9764, t9766, t9768, t9770, t9771)
}
