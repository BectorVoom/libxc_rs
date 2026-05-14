//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 947/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk947<F: Float>(t44: F, t51: F, t1213: F, t1216: F, t1219: F, t2466: F, t2469: F, t40: F, t48: F, t6976: F, t6979: F, t6980: F, t4948: F, t893: F, t1368: F, t35: F, t419: F, t1225: F, t1228: F, t2474: F, t2477: F, t53: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t6990 = piecewise3(t45, 0.0, -8.0 / 27.0 * t6976 * t1213 + 16.0 / 9.0 * t6979 * t6980 + 4.0 / 9.0 * t2466 * t1219 + 8.0 / 3.0 * t48 * t1216 - 8.0 * t2469 * t40);
    let t6991 = t4948 * t893;
    let t6994 = t1368 * t35;
    let t6995 = t1216 * t419;
    let t7005 = piecewise3(t52, 0.0, -8.0 / 27.0 * t6991 * t1225 - 16.0 / 9.0 * t6994 * t6995 + 4.0 / 9.0 * t2474 * t1228 - 8.0 / 3.0 * t53 * t1216 + 8.0 * t2477 * t40);
    (t6990, t6991, t6994, t6995, t7005)
}
