//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 855/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk855<F: Float>(t6887: F, t766: F, t2332: F, t287: F, t4881: F, t4886: F, t4896: F, t2850: F, t797: F, t1527: F, t2788: F, t4983: F, t2461: F, t879: F, t2321: F, t955: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6888 = t6887 * t766;
    let t6897 = 1.0 / t2332 / t287;
    let t6946 = 12.0 * t4881;
    let t6948 = 80.0 * t4886;
    let t6951 = 32.0 * t4896;
    let t6955 = t2850 * t797;
    let t6959 = t2788 * t1527;
    let t6961 = 48.0 * t4983;
    let t6963 = 2.0 * t879 * t2461;
    let t6966 = t2321 * t955;
    (t6888, t6897, t6946, t6948, t6951, t6955, t6959, t6961, t6963, t6966)
}
