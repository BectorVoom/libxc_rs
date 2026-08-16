//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 536/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk536(t1973: f64, t7255: f64, t236: f64, t4564: f64, t1971: f64, t1970: f64, t325: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t7256 = t7255 * t1973;
    let t7257 = 0.85129199786595678796e-5_f64 * t7256;
    let t7258 = t236 * t4564;
    let t7259 = t1971 * t7258;
    let t7260 = t1970 * t7259;
    let t7261 = 0.42564599893297839398e-5_f64 * t7260;
    let t7262 = t325 * t874;
    (t7257, t7259, t7261, t7262)
}
