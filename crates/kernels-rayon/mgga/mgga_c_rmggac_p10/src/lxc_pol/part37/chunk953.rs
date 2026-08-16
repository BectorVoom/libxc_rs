//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 953/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk953(t74913: f64, t74915: f64, t74917: f64, t74921: f64, t15489: f64, t16156: f64, t14385: f64, t39277: f64, t2144: f64, t2447: f64, t507: f64, t2136: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77254 = 0.2553875993597870364e-4_f64 * t74913;
    let t77255 = 0.2553875993597870364e-4_f64 * t74915;
    let t77256 = 0.79828278012425390427e-1_f64 * t74917;
    let t77258 = 0.10227998120342003148e-1_f64 * t74921;
    let t77259 = t16156 * t15489;
    let t77260 = 0.19863479950205658386e-4_f64 * t77259;
    let t77264 = t39277 * t14385;
    let t77265 = 0.53205749866622299248e-5_f64 * t77264;
    let t77269 = t507 * t2144 * t2447;
    let t77270 = t77269 * t2136;
    (t77254, t77255, t77256, t77258, t77260, t77265, t77270)
}
