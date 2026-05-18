//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 926/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk926<F: Float>(t1469: F, t34976: F, t39851: F, t571: F, t39857: F, t8417: F, t8450: F, t10102: F, t34847: F, t1528: F, t3351: F, t511: F, t558: F, t7231: F) -> (F, F, F, F) {
    let t45436 = t39851 * t34976 * t571 * t1469;
    let t45439 = t8450 * t39857 * t8417;
    let t45441 = t34847 * t10102;
    let t45446 = t3351 * t7231 * t511 * t1528 * t558;
    (t45436, t45439, t45441, t45446)
}
