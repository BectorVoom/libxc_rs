//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 587/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk587<F: Float>(t1801: F, t6714: F, t1800: F, t6713: F, t2575: F, sigma2: F) -> (F, F, F, F) {
    let t6715 = t1801 * t6714;
    let t6716 = t1800 * t6715;
    let t6717 = t6713 * t6716;
    let t6719 = t2575 * sigma2;
    (t6715, t6716, t6717, t6719)
}
