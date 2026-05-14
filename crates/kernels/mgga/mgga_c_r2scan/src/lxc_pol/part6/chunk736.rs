//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 736/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk736<F: Float>(t44: F, t51: F, t1361: F, t415: F, t1219: F, t48: F, t4905: F, t4913: F, t4938: F, t141: F, t1368: F, t419: F, t1228: F, t4921: F, t4927: F, t53: F, t60: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t4941 = t1361 * t415;
    let t4947 = piecewise3(t45, 0.0, -8.0 / 27.0 * t4938 * t4905 + 4.0 / 3.0 * t4941 * t1219 + 4.0 / 3.0 * t48 * t4913);
    let t4948 = 1.0 / t141;
    let t4951 = t1368 * t419;
    let t4957 = piecewise3(t52, 0.0, -8.0 / 27.0 * t4948 * t4921 + 4.0 / 3.0 * t4951 * t1228 + 4.0 / 3.0 * t53 * t4927);
    let t4958 = t4947 + t4957;
    let t4959 = t4958 * t60;
    (t4948, t4958, t4959)
}
