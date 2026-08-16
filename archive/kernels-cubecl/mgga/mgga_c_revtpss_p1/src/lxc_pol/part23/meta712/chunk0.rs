//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2470/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2470<F: Float>(t14238: F, t2453: F, t10142: F, t10073: F, t14231: F, t10139: F, t14219: F, t9285: F, t14215: F, t2470: F, t4101: F, t14220: F, t46495: F) -> (F, F, F, F, F, F) {
    let t48007 = t2453 * t14238;
    let t48008 = t48007 * t10142;
    let t48009 = F::cast_from(0.34697458558045176417e-2_f64) * t48008;
    let t48029 = t10073 * t14231;
    let t48036 = t10139 * t14219 * t9285;
    let t48039 = t4101 * t14215 * t2470;
    let t48040 = F::cast_from(0.39029762157531132076e-1_f64) * t48039;
    let t48041 = t46495 * t14220;
    (t48007, t48009, t48029, t48036, t48040, t48041)
}
