//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1233/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1233<F: Float>(t2184: F, t2185: F, t551: F, t6343: F, t2215: F, t6422: F, t1620: F, t5125: F, t545: F, t7613: F, t560: F, t6107: F, t2148: F, t6395: F, t6403: F, t1550: F) -> (F, F, F, F, F, F, F) {
    let t22861 = t2184 * t551 * t6343 * t2185;
    let t22863 = t6422 * t2215;
    let t22865 = t1620 * t5125;
    let t22868 = t545 * t7613;
    let t22869 = t6107 * t560;
    let t22871 = t22868 * t2148 * t22869;
    let t22873 = t6395 * t6403;
    let t22875 = t560 * t1550;
    (t22861, t22863, t22865, t22868, t22871, t22873, t22875)
}
