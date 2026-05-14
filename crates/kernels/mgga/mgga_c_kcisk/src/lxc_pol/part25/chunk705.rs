//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 705/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk705<F: Float>(t772: F, t7624: F, t782: F, t2020: F, t2642: F, t1636: F, t1775: F, t5507: F, t2023: F, t7261: F, t7528: F) -> (F, F, F, F, F, F, F, F) {
    let t783 = 0.0 < t772;
    let t7625 = t782 * t7624;
    let t7627 = t2020 * t2642;
    let t7628 = t7627 * t1636;
    let t7629 = t1775 * t7628;
    let t7632 = t5507 * t2642;
    let t7633 = t7632 * t2023;
    let t7634 = t7261 * t7633;
    let t7638 = piecewise3(t783, t7528, -t7528);
    (t7625, t7627, t7628, t7629, t7632, t7633, t7634, t7638)
}
