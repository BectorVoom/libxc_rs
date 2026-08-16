//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 982/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk982(t14385: f64, t39277: f64, t2144: f64, t2447: f64, t507: f64, t2136: f64, t235: f64, t7190: f64, t2141: f64, t7262: f64, t2147: f64, t74943: f64) -> (f64, f64, f64, f64, f64) {
    let t77264 = t39277 * t14385;
    let t77265 = 0.53205749866622299248e-5_f64 * t77264;
    let t77269 = t507 * t2144 * t2447;
    let t77270 = t77269 * t2136;
    let t77271 = 0.10227998120342003148e-1_f64 * t77270;
    let t77273 = t235 * t7190 * t2447;
    let t77274 = t77273 * t2141;
    let t77275 = 0.13637330827122670864e-1_f64 * t77274;
    let t77277 = t235 * t7262 * t2447;
    let t77278 = t77277 * t2147;
    let t77279 = 0.68186654135613354322e-2_f64 * t77278;
    let t77280 = 0.2553875993597870364e-4_f64 * t74943;
    (t77265, t77271, t77275, t77279, t77280)
}
