//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 619/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk619<F: Float>(t3308: F, t345: F, t975: F, t1102: F, t278: F, t3038: F, t3253: F, t3256: F, t3258: F, t3260: F, t3265: F, t3271: F, t3276: F, t3281: F, t3285: F, t3290: F, t3296: F, t3300: F, t3305: F, t344: F) -> (F, F) {
    let t3309 = t345 * t3308;
    let t3312 = t975 * t975;
    let t3316 = -t3253 + F::cast_from(0.8760572888888888889e-3_f64) * t3256 + F::cast_from(0.19711289e-2_f64) * t3258 - F::cast_from(0.13140859333333333333e-2_f64) * t3260 + F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t3265 + F::cast_from(0.19711289e-2_f64) * t1102 * t3271 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t3276 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t3281 + F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t3285 + F::cast_from(0.7391733375e-3_f64) * t344 * t3290 - F::cast_from(0.295669335e-2_f64) * t1102 * t3296 + F::cast_from(0.1478346675e-2_f64) * t344 * t3300 + F::cast_from(0.19711289e-2_f64) * t344 * t3305 - F::cast_from(0.98556445e-3_f64) * t344 * t3309 - F::cast_from(4.0_f64) * t3312 - F::cast_from(4.0_f64) * t278 * t3038;
    (t3309, t3316)
}
