//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 733/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk733<F: Float>(t16963: F, t379: F, t2221: F, t1882: F, t4726: F, t4805: F, t558: F, t574: F, t605: F, t11593: F, t12617: F, t12620: F, t12642: F, t12644: F, t12670: F, t12672: F, t12674: F, t16943: F, t16947: F, t16952: F, t16957: F, t16960: F, t1901: F, t446: F) -> (F,) {
    let t16964 = t16963 * t379;
    let t16965 = t2221 * t16964;
    let t16969 = t1882 * t4726;
    let t16971 = t4805 * t558;
    let t16973 = t574 * t605 * t16971;
    let t16976 = -2.0 / 3.0 * t1901 * t16943 + 8.0 / 9.0 * t11593 * t16947 - 2.0 / 9.0 * t1901 * t16952 - 2.0 / 9.0 * t1901 * t16957 + 2.0 / 9.0 * t1901 * t16960 + t1901 * t16965 / 9.0 - 8.0 / 81.0 * t12617 + t12620 - 2.0 / 9.0 * t16969 - t12642 - t12644 + t12670 + t12672 + t12674 + t446 * t16973 / 3.0;
    (t16976,)
}
