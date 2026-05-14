//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1078/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1078<F: Float>(t17708: F, t342: F, t630: F, t240: F, t43194: F, t1526: F, t3713: F, t9483: F, t42262: F, t4906: F, t3695: F, t17698: F, t52679: F, t1255: F, t2770: F, t10478: F) -> (F, F, F, F, F, F, F, F) {
    let t69081 = t342 * t630 * t17708 / 6.0;
    let t69108 = t43194 * t240;
    let t69132 = t1526 * t9483 * t3713 / 18.0;
    let t69137 = t1526 * t42262 * t4906;
    let t69141 = t1526 * t9483 * t3695 / 18.0;
    let t69143 = t1526 * t52679 * t17698;
    let t69875 = t2770 * t1255;
    let t69879 = t10478 * t1255;
    (t69081, t69108, t69132, t69137, t69141, t69143, t69875, t69879)
}
