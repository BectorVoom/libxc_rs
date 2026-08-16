//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2157/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2157(t13184: f64, t841: f64, t87407: f64, t23083: f64, t25123: f64, t81912: f64, t81887: f64, t81889: f64, t81899: f64, t81903: f64, t81909: f64, t87379: f64, t87381: f64, t87387: f64, t87389: f64, t87391: f64, t87395: f64, t87399: f64, t87402: f64, t87403: f64, t87405: f64) -> f64 {
    let t87409 = t87407 * t841 * t13184;
    let t87411 = t23083 * t25123;
    let t87412 = 0.28260929265898273598e-2_f64 * t87411;
    let t87414 = 0.22608743412718618878e-1_f64 * t81912;
    let t87415 = t87379 / 384.0_f64 + t87381 / 384.0_f64 - 7.0_f64 / 1152.0_f64 * t81887 + 7.0_f64 / 2304.0_f64 * t81889 + 0.20186378047070195427e-3_f64 * t81899 + 0.20186378047070195427e-3_f64 * t81903 - 0.31625325607076639502e-2_f64 * t87387 + 5.0_f64 / 192.0_f64 * t87389 + 5.0_f64 / 384.0_f64 * t87391 + 0.24223653656484234512e-2_f64 * t87395 + 0.12111826828242117256e-2_f64 * t87399 - t87402 + 119.0_f64 / 6912.0_f64 * t87403 - 0.52708876011794399171e-3_f64 * t87405 - 0.67826230238155856634e-1_f64 * t87409 + t87412 + 0.14130464632949136799e-2_f64 * t81909 - t87414;
    t87415
}
