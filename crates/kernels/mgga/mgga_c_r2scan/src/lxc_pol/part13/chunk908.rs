//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 908/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk908<F: Float>(t10924: F, t10933: F, t10937: F, t10945: F, t10948: F, t10952: F, t10957: F, t10960: F, t10965: F, t10970: F, t10974: F, t10983: F, t11534: F, t11542: F, t11547: F, t1102: F, t3314: F, t3692: F) -> (F, F) {
    let t11614 = t11534 + t10924 - t10933 + 0.96056421943322389208e-3 * t10937 + t10945 + t10948 + 0.21684485328539747656e-4 * t10952 + t10957 - 0.15243824895787514157e-3 * t10960 - t10965 + t11542 + t10970 + t10974 - t10983 + t11547;
    let t11616 = t1102 * t3314 * t3692;
    (t11614, t11616)
}
