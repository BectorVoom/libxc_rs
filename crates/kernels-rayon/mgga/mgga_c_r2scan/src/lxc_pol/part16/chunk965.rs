//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 965/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk965(t10924: f64, t10933: f64, t10937: f64, t10945: f64, t10948: f64, t10952: f64, t10957: f64, t10960: f64, t10965: f64, t10970: f64, t10974: f64, t10983: f64, t11534: f64, t11542: f64, t11547: f64) -> f64 {
    let t11614 = t11534 + t10924 - t10933 + 0.96056421943322389208e-3_f64 * t10937 + t10945 + t10948 + 0.21684485328539747656e-4_f64 * t10952 + t10957 - 0.15243824895787514157e-3_f64 * t10960 - t10965 + t11542 + t10970 + t10974 - t10983 + t11547;
    t11614
}
