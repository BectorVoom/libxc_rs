//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 952/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk952<F: Float>(t31662: F, t1086: F, t7605: F, t1998: F, t3531: F, t7528: F, t7637: F, t2109: F, t7610: F, t1980: F, t31025: F, t7458: F) -> (F, F, F, F, F, F) {
    let t31663 = F::new(0.38586616306262763276e-2) * t31662;
    let t31676 = t7605 * t1086;
    let t31680 = t1998 * t3531;
    let t31682 = t7637 * t7528;
    let t31684 = t7610 * t2109;
    let t31687 = t1980 * t7458 * t31025;
    (t31663, t31676, t31680, t31682, t31684, t31687)
}
