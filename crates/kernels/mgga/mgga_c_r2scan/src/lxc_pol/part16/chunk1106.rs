//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1106/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1106<F: Float>(t481: F, t9573: F, t3262: F, t3263: F, t38282: F, t38298: F, t38303: F, t38312: F, t38323: F, t40642: F, t40659: F, t40672: F, t43939: F, t43943: F, t43946: F, t43949: F, t43953: F, t43958: F) -> (F, F) {
    let t43959 = t9573 * t481;
    let t43962 = 3.0 / 2.0 * t3262 * t3263 * t43959;
    let t43963 = 0.60975299583150056628e-3 * t40642 - t38282 + 0.68400385060046895006e-6 * t40659 - 0.15243824895787514157e-3 * t43939 + 0.21684485328539747656e-4 * t43943 - t43946 - t43949 - t43953 - 0.70441376091769752087e-2 * t40672 - t38298 - 0.72042316457491791906e-3 * t38303 + t38312 - t43958 - t43962 + t38323;
    (t43962, t43963)
}
