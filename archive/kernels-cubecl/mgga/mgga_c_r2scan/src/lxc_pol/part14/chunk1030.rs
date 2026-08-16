//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1030/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1030<F: Float>(t146: F, t2078: F, t2145: F, t1543: F, t6212: F, t133: F, t5052: F, t10878: F, t545: F, t128: F, t20094: F, t2182: F, t3303: F) -> (F, F, F, F, F, F) {
    let t20825 = t146 * t2145 * t2078;
    let t20853 = t6212 * t1543;
    let t20946 = t5052 * t133;
    let t22731 = t545 * t10878;
    let t22766 = t20094 * t128;
    let t22790 = t2182 * t3303;
    (t20825, t20853, t20946, t22731, t22766, t22790)
}
