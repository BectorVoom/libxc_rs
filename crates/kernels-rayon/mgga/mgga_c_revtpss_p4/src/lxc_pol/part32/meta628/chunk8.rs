//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2017/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2017(t103286: f64, t106030: f64, t106033: f64, t106035: f64, t106037: f64, t106040: f64, t106042: f64, t106044: f64, t106046: f64, t106048: f64, t106050: f64, t106053: f64, t99013: f64) -> f64 {
    let t110406 = t103286 - 0.57165357490759649296e-4_f64 * t106030 + 0.28582678745379824648e-4_f64 * t106033 + 0.43366402397256813419e-2_f64 * t99013 - 0.34299214494455789578e-2_f64 * t106035 - 0.2032800112371413129e-3_f64 * t106037 + 0.28582678745379824648e-4_f64 * t106040 + 0.40015750243531754507e-2_f64 * t106042 - 0.34299214494455789578e-1_f64 * t106044 - 0.13719685797782315831e-1_f64 * t106046 - 0.50820002809285328225e-4_f64 * t106048 + 0.10164000561857065645e-3_f64 * t106050 - 0.22866142996303859718e-3_f64 * t106053;
    t110406
}
