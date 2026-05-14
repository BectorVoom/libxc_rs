//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 653/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk653<F: Float>(t2920: F, t308: F, t1001: F, t1268: F, t2901: F, t2905: F, t2911: F, t2917: F, t295: F, t305: F, t309: F, t997: F, t1010: F) -> (F, F, F) {
    let t2921 = t308 * t2920;
    let t2924 = 10.0 / 9.0 * t295 * t2901 + 5.0 / 3.0 * t295 * t2905 + 40.0 / 9.0 * t2911 * t309 - 50.0 / 9.0 * t997 * t1001 + 10.0 / 9.0 * t305 * t2917 + 5.0 / 3.0 * t305 * t2921 - t1268;
    let t2928 = t1010 * t1010;
    (t2921, t2924, t2928)
}
