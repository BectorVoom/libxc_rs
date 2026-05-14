//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1270/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1270<F: Float>(t51: F, t1216: F, t1225: F, t1228: F, t419: F, t40: F, t2477: F, t409: F, t1368: F, t18814: F, t2474: F, t35: F, t4921: F, t4927: F, t4948: F, t53: F, t6991: F, t6994: F, t893: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t23869 = t1216 * t1225;
    let t23872 = t419 * t1228;
    let t23878 = t40 * t419;
    let t23881 = t1216 * t1228;
    let t23889 = 32.0 * t2477 * t409;
    let t23891 = piecewise3(t52, 0.0, 40.0 / 81.0 * t18814 * t893 * t4921 + 16.0 / 9.0 * t4948 * t35 * t23869 - 8.0 / 9.0 * t6991 * t23872 - 8.0 / 3.0 * t1368 * t1216 * t419 + 8.0 * t6994 * t23878 - 8.0 / 3.0 * t6994 * t23881 + 4.0 / 9.0 * t2474 * t4927 + 16.0 * t53 * t40 - t23889);
    (t23869, t23872, t23878, t23881, t23891)
}
