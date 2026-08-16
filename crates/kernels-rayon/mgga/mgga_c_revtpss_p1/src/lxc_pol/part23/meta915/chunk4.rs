//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2953/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2953(t11409: f64, t11507: f64, t15104: f64, t15266: f64, t15406: f64, t19263: f64, t19266: f64, t19269: f64, t23706: f64, t311: f64, t52812: f64, t6205: f64, t77598: f64, t78319: f64, t78322: f64, t78325: f64, t78328: f64, t78332: f64, t78335: f64, t78339: f64, t78342: f64, t78375: f64, t78394: f64, t953: f64, t972: f64) -> f64 {
    let t78398 = -t78319 + t78322 + t78325 + t78328 - t78332 - t78335 - t78339 - t78342 + 18.0_f64 * t15406 * t19263 - 12.0_f64 * t15104 * t19266 - 0.57895126195293126241e3_f64 * t52812 * t19269 - 24.0_f64 * t11409 * t23706 * t953 - 0.19751673498613801407e-1_f64 * t77598 + 0.30762056574649219973e4_f64 * t11507 * t6205 * t15266 * t972 - 0.310907e-1_f64 * (t78375 + t78394) * t311;
    t78398
}
