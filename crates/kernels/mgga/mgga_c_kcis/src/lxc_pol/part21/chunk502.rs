//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 502/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk502<F: Float>(t313: F, t934: F, t1045: F, t3293: F, t1109: F, t2952: F, t345: F, t1035: F, t346: F, t3074: F, t1114: F, t3096: F, t975: F, t1102: F, t278: F, t3038: F, t3253: F, t3256: F, t3258: F, t3260: F, t3265: F, t3271: F, t3276: F, t3281: F, t3285: F, t3290: F, t344: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3294 = t313 * t934;
    let t3295 = t3294 * t1045;
    let t3296 = t3293 * t3295;
    let t3299 = t1109 * t2952;
    let t3300 = t345 * t3299;
    let t3303 = t346 * t1035;
    let t3304 = t3303 * t3074;
    let t3305 = t345 * t3304;
    let t3308 = t1114 * t3096;
    let t3309 = t345 * t3308;
    let t3312 = t975 * t975;
    let t3316 = -t3253 + 0.8760572888888888889e-3 * t3256 + 0.19711289e-2 * t3258 - 0.13140859333333333333e-2 * t3260 + 0.10950716111111111111e-2 * t1102 * t3265 + 0.19711289e-2 * t1102 * t3271 - 0.13140859333333333333e-2 * t1102 * t3276 - 0.13140859333333333333e-2 * t1102 * t3281 + 0.65704296666666666667e-3 * t1102 * t3285 + 0.7391733375e-3 * t344 * t3290 - 0.295669335e-2 * t1102 * t3296 + 0.1478346675e-2 * t344 * t3300 + 0.19711289e-2 * t344 * t3305 - 0.98556445e-3 * t344 * t3309 - 4.0 * t3312 - 4.0 * t278 * t3038;
    (t3295, t3296, t3299, t3300, t3303, t3304, t3305, t3308, t3309, t3316)
}
