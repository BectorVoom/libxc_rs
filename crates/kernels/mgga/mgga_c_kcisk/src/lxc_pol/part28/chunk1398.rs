//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1398/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1398<F: Float>(t17775: F, t34310: F, t17772: F, t9988: F, t34303: F, t2594: F, t34374: F, t5218: F, t1957: F, t35274: F, t47033: F, t11701: F, t35277: F, t1800: F, t24269: F, t1950: F, t22250: F) -> (F, F, F, F, F, F, F, F) {
    let t122156 = 4.0 * t17775 * t34310;
    let t122158 = 2.0 * t17772 * t9988;
    let t122160 = 4.0 * t17775 * t34303;
    let t122163 = 4.0 * t5218 * t34374 * t2594;
    let t122166 = 24.0 * t47033 * t35274 * t1957;
    let t122169 = 12.0 * t11701 * t35277 * t1957;
    let t122170 = t1800 * t24269;
    let t122172 = t22250 * t1950;
    (t122156, t122158, t122160, t122163, t122166, t122169, t122170, t122172)
}
