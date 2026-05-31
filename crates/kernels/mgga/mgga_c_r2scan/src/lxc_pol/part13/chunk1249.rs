//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1249/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1249<F: Float>(t1039: F, t1044: F, t11028: F, t1108: F, t11166: F, t12019: F, t40666: F, t40670: F, t40679: F, t40683: F, t40686: F, t40690: F, t40694: F, t40699: F, t40704: F, t40708: F, t40711: F, t40715: F, t40717: F, t8505: F, t885: F) -> F {
    let t41098 = t1039 * t11028 + t1044 * t11166 + t1108 * t8505 + F::cast_from(2.0_f64) * t12019 * t885 + t40666 - t40670 + t40679 + t40683 + t40686 + t40690 - t40694 + t40699 - t40704 - t40708 + t40711 + t40715 - t40717;
    t41098
}
