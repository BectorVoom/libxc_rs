//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 738/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk738<F: Float>(t2679: F, t7354: F, t2684: F, t1844: F, t2465: F, t2464: F, t825: F, t6125: F, t549: F, t7222: F, t2021: F, t2026: F, t2554: F, t900: F) -> (F, F, F, F, F, F) {
    let t7355 = t7354 * t2679;
    let t7356 = t2684 * t7355;
    let t7358 = t2465 * t1844;
    let t7359 = t2464 * t7358;
    let t7360 = t825 * t7359;
    let t7362 = t2465 * t6125;
    let t7363 = t2464 * t7362;
    let t7364 = t2684 * t7363;
    let t7366 = t549 * t7222;
    let t7371 = t2021 * t2026;
    let t7372 = t900 * t2554;
    (t7356, t7360, t7364, t7366, t7371, t7372)
}
