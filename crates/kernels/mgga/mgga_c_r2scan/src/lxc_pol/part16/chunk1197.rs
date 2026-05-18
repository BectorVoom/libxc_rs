//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1197/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1197<F: Float>(t2184: F, t30213: F, t3308: F, t12547: F, t6425: F, t1592: F, t27996: F, t28000: F, t30292: F, t6449: F, t30296: F, t6528: F) -> (F, F, F, F, F, F) {
    let t43294 = t2184 * t3308 * t30213;
    let t43296 = t6425 * t12547;
    let t43299 = t1592 * t3308 * t27996;
    let t43302 = t1592 * t3308 * t28000;
    let t43305 = t6449 * t3308 * t30292;
    let t43308 = t6528 * t3308 * t30296;
    (t43294, t43296, t43299, t43302, t43305, t43308)
}
