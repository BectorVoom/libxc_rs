//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 990/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk990<F: Float>(t1070: F, t8355: F, t3363: F, t8358: F, t2378: F, t3366: F, t3629: F, t6654: F, t6661: F, t1010: F, t263: F, t826: F) -> (F, F, F, F, F, F, F) {
    let t11872 = t8355 * t1070;
    let t11874 = t8358 * t3363;
    let t11876 = t2378 * t3366;
    let t11878 = t6654 * t3629;
    let t11880 = t6661 * param_eta;
    let t11881 = t263 * t1010;
    let t11882 = t11881 * t826;
    (t11872, t11874, t11876, t11878, t11880, t11881, t11882)
}
