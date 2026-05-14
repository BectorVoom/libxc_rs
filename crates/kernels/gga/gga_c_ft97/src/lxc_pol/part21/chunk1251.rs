//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1251/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1251<F: Float>(t23405: F, t30145: F, t104462: F, t3483: F, t104321: F, t104426: F, t1349: F, t16988: F, t1969: F, t23400: F, t23413: F, t24080: F, t24081: F, t26581: F, t26783: F, t26809: F, t28: F, t30112: F, t30288: F, t30290: F, t3052: F, t3424: F, t4668: F, t4837: F, t558: F, t5766: F, t5772: F, t5778: F, t614: F, t6622: F, t94208: F) -> (F, F) {
    let t119257 = t23405 * t30145;
    let t119270 = t104462 * t3483;
    let t119282 = 4.0 / 9.0 * t26809 * t24080 * t24081 * t16988 + 2.0 / 9.0 * t5772 * t24080 * t104321 * t3424 - 2.0 / 3.0 * t5766 * t30112 + t119257 / 54.0 - t1349 * t28 * t5778 * t4837 * t558 / 3.0 - 2.0 / 9.0 * t26809 * t1969 * t26783 * t3052 - t23413 * t30145 / 18.0 + 8.0 * t119270 + t26581 * t6622 / 3.0 + t5766 * t30290 + t1349 * t28 * t94208 * t30288 + t1349 * t28 * t23400 * t614 * t4668 + t104426;
    (t119270, t119282)
}
