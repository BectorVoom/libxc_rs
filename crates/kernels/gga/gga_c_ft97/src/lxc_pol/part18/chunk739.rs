//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 739/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk739<F: Float>(t12664: F, t2180: F, t144: F, t1882: F, t3584: F, t3580: F, t3571: F, t3442: F, t8392: F, t3420: F, t9099: F, t1045: F, t582: F) -> (F, F, F, F, F, F, F) {
    let t12665 = t12664 * t2180;
    let t12666 = t144 * t12665;
    let t12670 = 2.0 / 9.0 * t1882 * t3584;
    let t12672 = 2.0 / 9.0 * t1882 * t3580;
    let t12674 = 2.0 / 9.0 * t1882 * t3571;
    let t12676 = 4.0 / 81.0 * t8392 * t3442;
    let t12677 = t9099 * t3420;
    let t12680 = t582 * t1045;
    (t12666, t12670, t12672, t12674, t12676, t12677, t12680)
}
