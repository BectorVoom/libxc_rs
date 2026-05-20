//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2196/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2196<F: Float>(t30056: F, t686: F, t72: F, t7289: F, t108280: F, t108282: F, t108294: F, t108296: F, t108302: F, t1444: F, t22433: F, t25921: F, t25924: F, t27837: F, t27903: F, t30017: F, t30020: F, t30021: F, t30101: F, t7279: F, t7295: F, t7304: F, t94662: F, t97843: F, t97847: F) -> (F, F) {
    let t108307 = t30056 * t72 * t686;
    let t108308 = t7289 * t108307;
    let t108310 = -F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t30020 * t1444 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t27903 - F::cast_from(0.12851425765524037203e-1_f64) * t108280 + F::cast_from(0.4336814094102599731e0_f64) * t108282 * t7304 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t30101 + F::cast_from(0.39029762157531132076e-1_f64) * t97843 + F::cast_from(0.96373646535613327359e-3_f64) * t97847 + F::cast_from(0.19274729307122665471e-1_f64) * t94662 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t30021 + F::cast_from(0.43368140941025997311e-1_f64) * t108294 - F::cast_from(0.77108554593144223219e-1_f64) * t108296 - F::cast_from(0.39512695097613069591e1_f64) * t7279 * t22433 - F::cast_from(0.54878743191129263322e-2_f64) * t108302 - F::cast_from(0.26020884564615598386e1_f64) * t25921 * t30017 - F::cast_from(0.12851425765524037203e-1_f64) * t108308;
    (t108307, t108310)
}
