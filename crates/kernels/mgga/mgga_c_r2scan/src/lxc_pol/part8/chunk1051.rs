//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1051/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1051<F: Float>(t322: F, t10468: F, t1013: F, t2944: F, t2941: F, t1299: F, t1300: F, t327: F, t6693: F, t834: F, t9679: F, t1018: F, t2951: F, t2953: F) -> (F, F, F, F, F, F, F) {
    let t324 = 0.0 < t322;
    let t332 = 0.25e1 < t322;
    let t10469 = piecewise3(t324, 0.0, t10468);
    let t10474 = t2944 * t1013;
    let t10479 = t1013 * t2941;
    let t10484 = -0.64e0 * t10469 * t327 - 0.384e1 * t9679 * t1013 - 0.384e1 * t10474 * t1299 - 0.384e1 * t6693 * t10474 - 0.384e1 * t1300 * t10479 - 0.64e0 * t834 * t10469;
    let t10486 = t2951 * t1018;
    let t10489 = t2953 * t1018;
    let t10492 = piecewise3(t332, 0.0, t10468);
    (t10469, t10474, t10479, t10484, t10486, t10489, t10492)
}
