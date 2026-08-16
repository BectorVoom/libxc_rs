//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1022/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1022<F: Float>(t1162: F, t1535: F, t17386: F, t4393: F, t4396: F, t1418: F, t3670: F, t1347: F, t1429: F, t3237: F, t5255: F, t997: F) -> (F, F, F, F, F, F) {
    let t17388 = t17386 * t1162 * t1535;
    let t17390 = t4396 * t4393;
    let t17392 = t3670 * t1418;
    let t17395 = t3670 * t1347;
    let t17397 = t3237 * t1429;
    let t17399 = t997 * t5255;
    (t17388, t17390, t17392, t17395, t17397, t17399)
}
