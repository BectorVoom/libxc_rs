//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 988/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk988<F: Float>(t3493: F, t792: F, t11002: F, t11336: F, t2262: F, t3270: F, t1120: F, t6692: F, t37038: F, t37075: F, t11217: F, t833: F, t1299: F, t3506: F, t11310: F, t1338: F) -> (F, F, F, F, F, F, F, F) {
    let t38770 = t3493 * t792;
    let t38771 = t11002 * t38770;
    let t38775 = t3270 * t11336 * t2262;
    let t38783 = t1120 * t6692;
    let t38792 = 308.0 / 27.0 * t37038;
    let t38808 = 308.0 / 27.0 * t37075;
    let t38834 = t11217 * t833;
    let t38839 = t3506 * t1299;
    let t38953 = t1338 * t11310;
    (t38771, t38775, t38783, t38792, t38808, t38834, t38839, t38953)
}
