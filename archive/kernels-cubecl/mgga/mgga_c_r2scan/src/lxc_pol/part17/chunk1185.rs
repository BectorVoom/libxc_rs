//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1185/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1185<F: Float>(t11670: F, t2124: F, t29764: F, t11705: F, t7313: F, t11708: F, t8240: F, t12538: F, t6395: F, t10868: F, t2147: F, t9292: F) -> (F, F, F, F, F) {
    let t43512 = t11670 * t2124 * t29764;
    let t43514 = t7313 * t11705;
    let t43516 = t8240 * t11708;
    let t43518 = t6395 * t12538;
    let t43521 = t2147 * t10868 * t9292;
    (t43512, t43514, t43516, t43518, t43521)
}
