//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 995/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk995<F: Float>(t10610: F, t12215: F, t11486: F, t3472: F, t3262: F, t3275: F, t3465: F, t7040: F, t11008: F, t11378: F, t11379: F, t11380: F, t11454: F, t11616: F, t12110: F, t12112: F, t12201: F, t12205: F, t12208: F, t12212: F, t12214: F) -> (F, F, F, F, F) {
    let t12216 = t10610 * t12215;
    let t12217 = F::new(3.0) / F::new(2.0) * t12216;
    let t12219 = t3472 * t11486;
    let t12220 = t3262 * t12219;
    let t12221 = F::new(15.0) / F::new(16.0) * t12220;
    let t12223 = t3275 * t3465 * t7040;
    let t12224 = t12223 / F::new(4.0);
    let t12225 = -t11378 - t12110 - t12112 - F::cast_from(0.81300399444200075499e-3_f64) * t11616 + t11379 - t12201 + t11380 + t12205 + t12208 - t12212 - t12214 - t12217 - F::cast_from(0.81300399444200075499e-3_f64) * t11008 - t11454 + t12221 + t12224;
    (t12216, t12219, t12220, t12223, t12225)
}
