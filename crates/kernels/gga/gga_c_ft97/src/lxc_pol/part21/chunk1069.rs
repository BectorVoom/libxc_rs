//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1069/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1069<F: Float>(t1332: F, t7763: F, t26309: F, t8392: F, t1339: F, t1786: F, t1882: F, t26432: F, t26346: F, t26234: F, t6471: F, t8232: F, t6540: F, t26337: F, t26147: F, t463: F, t5710: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t102487 = t1332 * t7763;
    let t102508 = 4.0 / 27.0 * t8392 * t26309;
    let t102524 = t1786 * t1339;
    let t102533 = 2.0 / 9.0 * t1882 * t26432;
    let t102543 = 4.0 / 27.0 * t8392 * t26346;
    let t102549 = 2.0 / 9.0 * t1882 * t26234;
    let t102599 = t8232 * t6471;
    let t102614 = t8232 * t6540;
    let t102626 = 4.0 / 9.0 * t1882 * t26337;
    let t102664 = 2.0 / 9.0 * t1882 * t26147;
    let t102678 = t463 * t5710;
    (t102487, t102508, t102524, t102533, t102543, t102549, t102599, t102614, t102626, t102664, t102678)
}
