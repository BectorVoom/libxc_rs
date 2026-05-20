//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1977/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1977<F: Float>(t102420: F, t5722: F, t28780: F, t98041: F, t27899: F, t28845: F, t28894: F, t97802: F, t98380: F, t102320: F, t102324: F, t102325: F, t102656: F, t108244: F, t14224: F, t1444: F, t25921: F, t25924: F, t25930: F, t26304: F, t27837: F, t27868: F, t28806: F, t30279: F, t30282: F, t6895: F, t7295: F, t7506: F, t96374: F) -> F {
    let t109534 = t102420 * t5722;
    let t109536 = t98041 * t28780;
    let t109539 = t27899 * t28845;
    let t109553 = t97802 * t28894;
    let t109555 = t98380 * t28894;
    let t109563 = -t102320 - F::cast_from(0.19514881078765566037e-1_f64) * t109534 + F::cast_from(0.51405703062096148813e-1_f64) * t109536 + t102324 + F::cast_from(0.86736281882051994624e-1_f64) * t102325 + F::cast_from(0.14456046980341999104e-1_f64) * t109539 - F::cast_from(0.26020884564615598386e1_f64) * t25921 * t30279 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t7506 * t6895 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t30282 * t1444 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t28806 + t96374 - F::cast_from(0.14456046980341999104e-1_f64) * t109553 + F::cast_from(0.25702851531048074406e-1_f64) * t109555 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t102656 * t14224 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t26304 * t108244;
    t109563
}
