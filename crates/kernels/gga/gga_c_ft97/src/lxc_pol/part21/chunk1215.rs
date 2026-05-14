//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1215/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1215<F: Float>(t29789: F, t487: F, t29919: F, t8392: F, t1852: F, t3255: F, t6557: F, t1882: F, t29857: F, t16291: F, t22943: F, t103252: F, t103435: F, t103745: F, t103769: F, t103823: F, t15951: F, t15955: F, t15959: F, t16077: F, t16305: F, t1901: F, t1902: F, t1909: F, t23244: F, t23323: F, t29931: F, t3183: F, t379: F, t4458: F, t446: F, t47659: F, t47666: F, t83: F, t8372: F, t91539: F) -> (F, F, F) {
    let t118087 = t487 * t29789;
    let t118102 = t8392 * t29919;
    let t118108 = t1852 * t6557 * t3255;
    let t118112 = t1882 * t29857;
    let t118114 = t22943 * t16291;
    let t118130 = t103745 + t1901 * t1909 * t118087 * t379 / 9.0 + 2.0 / 9.0 * t1901 * t103823 * t3183 - 2.0 / 9.0 * t1901 * t8372 * t29931 - 2.0 / 9.0 * t1901 * t1902 * t23244 * t4458 - 2.0 / 81.0 * t118102 + t1901 * t23323 * t16077 / 9.0 + 4.0 / 3.0 * t446 * t83 * t118108 + t118112 / 9.0 + 2.0 / 3.0 * t446 * t83 * t118114 + t103769 + 4.0 / 9.0 * t47659 * t103252 * t15959 + 4.0 / 9.0 * t47659 * t91539 * t16305 + 4.0 / 9.0 * t47659 * t103435 * t15951 - 4.0 / 27.0 * t47666 * t103435 * t15955;
    (t118108, t118114, t118130)
}
