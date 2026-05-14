//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1314/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1314<F: Float>(t51: F, t3010: F, t419: F, t1216: F, t32168: F, t1217: F, t1368: F, t18814: F, t2474: F, t53: F, t6991: F, t6994: F, t8576: F, t8584: F, t9869: F, t9874: F, t32172: F, zeta_threshold: F) -> (F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t32178 = t3010 * t419;
    let t32181 = t1216 * t3010;
    let t32189 = -t32168;
    let t32193 = piecewise3(t52, 0.0, 40.0 / 81.0 * t18814 * t9869 * t419 + 16.0 / 9.0 * t8576 * t1217 - 8.0 / 9.0 * t6991 * t32178 - 8.0 / 3.0 * t6994 * t32181 + 4.0 / 3.0 * t2474 * t8584 + 4.0 / 9.0 * t1368 * t9874 * t419 + 4.0 / 3.0 * t53 * t32189);
    let t32194 = t32172 + t32193;
    (t32178, t32181, t32189, t32194)
}
