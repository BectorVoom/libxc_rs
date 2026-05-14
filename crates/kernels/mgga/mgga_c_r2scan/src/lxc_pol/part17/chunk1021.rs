//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1021/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1021<F: Float>(t3308: F, t574: F, t9147: F, t1054: F, t2139: F, t8752: F, t2133: F, t8736: F, t40194: F, t40195: F, t8756: F, t10772: F, t10810: F, t3100: F, t9151: F, t8779: F) -> (F, F, F, F, F, F, F) {
    let t43012 = t574 * t3308 * t9147;
    let t43015 = t2139 * t1054 * t8752;
    let t43018 = t2133 * t1054 * t8736;
    let t43021 = t40194 * t40195 * t8756;
    let t43026 = t10772 * t10810 * t3100;
    let t43029 = t574 * t3308 * t9151;
    let t43032 = t574 * t3308 * t8779;
    (t43012, t43015, t43018, t43021, t43026, t43029, t43032)
}
