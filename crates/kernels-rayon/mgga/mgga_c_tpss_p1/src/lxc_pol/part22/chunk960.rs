//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 960/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk960(t1193: f64, t8027: f64, t1170: f64, t3280: f64, t8017: f64, t1220: f64, t2376: f64, t339: f64, t1235: f64, t3256: f64, t789: f64, t3263: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10038 = 0.35089341735807877242e1_f64 * t1193 * t8027;
    let t10039 = t1170 * t3280;
    let t10042 = 0.5848223622634646207e0_f64 * t1193 * t8017;
    let t10077 = t339 * t1220 * t2376;
    let t10078 = t10077 * t1235;
    let t10081 = t339 * t3256 * t789;
    let t10082 = t10081 * t3263;
    (t10038, t10039, t10042, t10077, t10078, t10082)
}
