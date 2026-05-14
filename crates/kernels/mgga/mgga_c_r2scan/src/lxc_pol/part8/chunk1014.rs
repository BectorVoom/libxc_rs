//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1014/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1014<F: Float>(t5053: F, t9937: F, t2505: F, t3016: F, t490: F, t9880: F, t109: F, t111: F, t2504: F, t3042: F, t3046: F, t3049: F, t915: F, t917: F, t9929: F) -> (F, F, F, F) {
    let t9938 = t5053 * t9937;
    let t9941 = t2505 * t3016;
    let t9944 = t490 * t9880;
    let t9947 = 60.0 * t109 * t9938 + 3.0 * t109 * t9944 - t9929 * t111 - 36.0 * t2504 * t9941 + 9.0 * t3042 * t917 - 36.0 * t915 * t3046 + 9.0 * t915 * t3049;
    (t9938, t9941, t9944, t9947)
}
