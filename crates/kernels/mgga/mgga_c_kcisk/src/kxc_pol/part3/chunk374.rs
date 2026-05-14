//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 374/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk374<F: Float>(t695: F, t702: F, t1060: F, t1919: F, t1860: F, t673: F, t140: F, t1470: F, t1883: F, t1888: F, t1909: F, t1918: F, t479: F, t709: F, t725: F, t716: F) -> (F, F, F, F, F) {
    let t1920 = t702 * t695;
    let t1922 = t1919 * t1920 * t1060;
    let t1925 = t673 * t1860;
    let t1929 = 0.619125e-2 * t1909 * t709 + 0.9286875e-2 * t725 * t1883 - 0.619125e-2 * t725 * t1888 - t1918 - 0.26531111111111111111e-1 * t1470 * t1922 - 0.39796666666666666666e-1 * t140 * t479 * t1925;
    let t1930 = t1929 * t716;
    (t1920, t1922, t1925, t1929, t1930)
}
