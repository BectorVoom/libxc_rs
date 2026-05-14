//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1395/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1395<F: Float>(t33550: F, t9442: F, t32105: F, t9792: F, t20160: F, t33587: F, t9446: F, t33557: F, t3733: F, t415: F, t1333: F, t33495: F, t32176: F, t33460: F, t53214: F, t9808: F) -> (F, F, F, F, F, F, F) {
    let t114783 = 0.18518518518518518519e-1 * t33550 * t9442;
    let t114784 = t9792 * t32105;
    let t114790 = 0.69444444444444444446e-2 * t9446 * t20160 * t33587;
    let t114794 = t415 * t33557 * t3733;
    let t114796 = t1333 * t33495;
    let t114799 = 0.26805555555555555556e-2 * t33460 * t32176;
    let t114803 = t9446 * t53214 * t9808;
    (t114783, t114784, t114790, t114794, t114796, t114799, t114803)
}
