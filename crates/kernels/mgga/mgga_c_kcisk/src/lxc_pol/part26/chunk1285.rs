//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1285/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1285<F: Float>(t20160: F, t33587: F, t9446: F, t1333: F, t33495: F, t32176: F, t33460: F, t53214: F, t9808: F, t1458: F, t33616: F, t4534: F, t9878: F, t1610: F, t33975: F, t33961: F, t9532: F) -> (F, F, F, F, F, F, F, F) {
    let t114790 = 0.69444444444444444446e-2 * t9446 * t20160 * t33587;
    let t114796 = t1333 * t33495;
    let t114799 = 0.26805555555555555556e-2 * t33460 * t32176;
    let t114803 = t9446 * t53214 * t9808;
    let t114849 = t33616 * t1458;
    let t114859 = t9878 * t4534;
    let t114965 = t33975 * t1610;
    let t114982 = 0.34722222222222222222e-2 * t33961 * t9532;
    (t114790, t114796, t114799, t114803, t114849, t114859, t114965, t114982)
}
