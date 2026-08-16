//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1196/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1196(t2453: f64, t25398: f64, t10982: f64, t1949: f64, t9646: f64, t10985: f64, t25422: f64, t25335: f64, t9303: f64, t1959: f64, t41117: f64, t68: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93194 = t2453 * t25398;
    let t93206 = 0.19637199382202157274e-3_f64 * t9646 * t1949 * t10982;
    let t93210 = 0.46263278077393568556e-2_f64 * t25422 * t10985;
    let t93224 = 0.26019841438354088051e-2_f64 * t9303 * t25335;
    let t93231 = 0.81814717454467823679e-4_f64 * t41117 * t1959;
    let t93238 = t68 * t785;
    (t93194, t93206, t93210, t93224, t93231, t93238)
}
