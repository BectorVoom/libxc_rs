//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 995/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk995(t10610: f64, t12215: f64, t11486: f64, t3472: f64, t3262: f64, t3275: f64, t3465: f64, t7040: f64, t11008: f64, t11378: f64, t11379: f64, t11380: f64, t11454: f64, t11616: f64, t12110: f64, t12112: f64, t12201: f64, t12205: f64, t12208: f64, t12212: f64, t12214: f64) -> (f64, f64, f64, f64, f64) {
    let t12216 = t10610 * t12215;
    let t12217 = 3.0_f64 / 2.0_f64 * t12216;
    let t12219 = t3472 * t11486;
    let t12220 = t3262 * t12219;
    let t12221 = 15.0_f64 / 16.0_f64 * t12220;
    let t12223 = t3275 * t3465 * t7040;
    let t12224 = t12223 / 4.0_f64;
    let t12225 = -t11378 - t12110 - t12112 - 0.81300399444200075499e-3_f64 * t11616 + t11379 - t12201 + t11380 + t12205 + t12208 - t12212 - t12214 - t12217 - 0.81300399444200075499e-3_f64 * t11008 - t11454 + t12221 + t12224;
    (t12216, t12219, t12220, t12223, t12225)
}
