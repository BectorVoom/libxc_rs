//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1200/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1200(t94386: f64, t96220: f64, t94391: f64, t1358: f64, t212: f64, t26333: f64, t689: f64, t2097: f64, t9646: f64, t9648: f64, t1444: f64, t25921: f64, t26351: f64, t4131: f64, t7295: f64, t7296: f64, t7506: f64, t96188: f64, t96193: f64, t96195: f64, t96197: f64, t96206: f64, t96210: f64, t96211: f64, t96218: f64) -> (f64, f64) {
    let t96221 = t96220 * t94386;
    let t96222 = t94391 * t96221;
    let t96226 = t689 * t212 * t26333 * t1358;
    let t96230 = 0.19637199382202157274e-3_f64 * t9646 * t2097 * t9648;
    let t96231 = 0.86736281882051994623e-1_f64 * t96188 - 0.43368140941025997312e-1_f64 * t96193 + 0.77108554593144223218e-1_f64 * t96195 + 0.21951497276451705329e-1_f64 * t96197 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t26333 * t1444 + t96206 + 0.26020884564615598386e1_f64 * t25921 * t26351 - t96210 - 0.28912093960683998208e-1_f64 * t96211 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t7506 * t4131 - t96218 + 0.68549505033305214441e-2_f64 * t96222 - 0.16463622957338778996e-1_f64 * t96226 + t96230;
    (t96221, t96231)
}
