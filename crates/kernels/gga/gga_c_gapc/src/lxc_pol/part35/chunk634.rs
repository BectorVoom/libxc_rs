//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 634/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk634<F: Float>(t3755: F, t653: F, t2211: F, t442: F, t128: F, t818: F, t2716: F, t2188: F, t435: F, t188: F, t3: F, t761: F, t825: F, t285: F, t282: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6194 = t3755 * t653;
    let t6201 = t2211 * t442;
    let t6210 = t128 * t818;
    let t6773 = t2716 * t442;
    let t6791 = t435 * t2188;
    let t6803 = t3 * t188;
    let t6808 = t761 * t825;
    let t6849 = t285 * t285;
    let t6851 = 1.0 / t6849 / t282;
    (t6194, t6201, t6210, t6773, t6791, t6803, t6808, t6849, t6851)
}
