//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1206/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1206(t1232: f64, t1265: f64, t520: f64, t1258: f64, t3255: f64, t1270: f64, t3245: f64, t196: f64, t197: f64, t3174: f64, t508: f64, t1759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18499 = t1265 * t1232 * t520;
    let t18511 = t1258 * t3255;
    let t18539 = t1270 * t3245;
    let t18544 = t3174 * t196 * t197;
    let t18546 = t197 * t508;
    let t18547 = t1759 * t18546;
    (t18499, t18511, t18539, t18544, t18546, t18547)
}
