//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 448/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk448<F: Float>(t1394: F, t429: F, t431: F, t3812: F, t213: F, t442: F, t1390: F, t967: F, t167: F, t3532: F, t408: F, t1218: F, t411: F, t338: F, t389: F, t394: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3851 = 0.8197e-2 * t429 * t1394 * t431;
    let t3852 = 0.23911438650126355246e-1 * t3812;
    let t3857 = t213 * t442;
    let t3858 = 0.15538616723388920628e-3 * t3857;
    let t3859 = t967 * t1390;
    let t3891 = t167 * t3532;
    let t3923 = t408 * t408;
    let t3924 = 1.0 / t3923;
    let t3929 = 1.0 / t1218 / t411;
    let t3930 = t338 * t3929;
    let t3933 = t389 * t394;
    (t3851, t3852, t3858, t3859, t3891, t3923, t3924, t3929, t3930, t3933)
}
