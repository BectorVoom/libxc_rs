//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 944/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk944<F: Float>(t6787: F, t6796: F, t6800: F, t6904: F, t354: F, t4695: F, t2881: F, t860: F, t4881: F, t4883: F, t4886: F, t4892: F, t4894: F, t4896: F, t4898: F, t4703: F, t4880: F, t4891: F, t4901: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6906 = t6787 + t6796 + t6800 + t6904;
    let t6907 = t354 * t6906;
    let t6943 = 2.0 * t4695;
    let t6944 = t860 * t2881;
    let t6946 = 12.0 * t4881;
    let t6947 = 40.0 * t4883;
    let t6948 = 80.0 * t4886;
    let t6949 = 4.0 * t4892;
    let t6950 = 4.0 * t4894;
    let t6951 = 32.0 * t4896;
    let t6952 = 24.0 * t4898;
    let t6953 = -t6943 - t4880 + t6946 - t6947 - t6948 + t4891 + t6949 + t6950 - t4703 + t6951 + t6952 - t4901;
    (t6906, t6907, t6943, t6944, t6946, t6947, t6948, t6949, t6950, t6951, t6952, t6953)
}
