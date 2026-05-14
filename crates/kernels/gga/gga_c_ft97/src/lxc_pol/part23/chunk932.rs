//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 932/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk932<F: Float>(t10248: F, t28516: F, t446: F, t25140: F, t3886: F, t2665: F, t25037: F, t10409: F, t1486: F, t681: F, t7075: F, t1882: F, t7080: F, t668: F, t7021: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28517 = t10248 * t28516;
    let t28518 = t446 * t28517;
    let t28520 = t25140 * t3886;
    let t28521 = t2665 * t28520;
    let t28522 = t446 * t28521;
    let t28524 = t25037 * t3886;
    let t28525 = t10409 * t28524;
    let t28526 = t446 * t28525;
    let t28529 = t1486 * t681 * t7075;
    let t28531 = t1882 * t7080;
    let t28533 = t7021 * t668;
    (t28517, t28518, t28520, t28521, t28522, t28524, t28525, t28526, t28529, t28531, t28533)
}
