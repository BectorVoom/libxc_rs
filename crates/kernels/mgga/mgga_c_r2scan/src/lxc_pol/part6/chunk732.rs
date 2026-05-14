//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 732/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk732<F: Float>(t44: F, t1219: F, t472: F, t4904: F, t4905: F, t4908: F, t4913: F, t51: F, t53: F, t1225: F, t419: F, zeta_threshold: F) -> (F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t4917 = piecewise3(t45, 0.0, 8.0 / 27.0 * t4904 * t4905 - 2.0 / 3.0 * t4908 * t1219 + 2.0 / 3.0 * t472 * t4913);
    let t4918 = t51 * t51;
    let t4920 = 1.0 / t53 / t4918;
    let t4921 = t1225 * t419;
    (t4917, t4918, t4920, t4921)
}
