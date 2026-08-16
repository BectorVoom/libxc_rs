//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1182/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1182<F: Float>(t1058: F, t1060: F, t2201: F, t9365: F, t11760: F, t11764: F, t2207: F, t11805: F, t39378: F, t10772: F, t3308: F, t8833: F) -> (F, F, F, F) {
    let t43130 = t2201 * t1058 * t1060 * t9365;
    let t43133 = t2207 * t11760 * t11764;
    let t43135 = t39378 * t11805;
    let t43138 = t10772 * t3308 * t8833;
    (t43130, t43133, t43135, t43138)
}
