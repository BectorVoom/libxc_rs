//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 619/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk619(t3308: f64, t345: f64, t975: f64, t1102: f64, t278: f64, t3038: f64, t3253: f64, t3256: f64, t3258: f64, t3260: f64, t3265: f64, t3271: f64, t3276: f64, t3281: f64, t3285: f64, t3290: f64, t3296: f64, t3300: f64, t3305: f64, t344: f64) -> (f64, f64) {
    let t3309 = t345 * t3308;
    let t3312 = t975 * t975;
    let t3316 = -t3253 + 0.8760572888888888889e-3_f64 * t3256 + 0.19711289e-2_f64 * t3258 - 0.13140859333333333333e-2_f64 * t3260 + 0.10950716111111111111e-2_f64 * t1102 * t3265 + 0.19711289e-2_f64 * t1102 * t3271 - 0.13140859333333333333e-2_f64 * t1102 * t3276 - 0.13140859333333333333e-2_f64 * t1102 * t3281 + 0.65704296666666666667e-3_f64 * t1102 * t3285 + 0.7391733375e-3_f64 * t344 * t3290 - 0.295669335e-2_f64 * t1102 * t3296 + 0.1478346675e-2_f64 * t344 * t3300 + 0.19711289e-2_f64 * t344 * t3305 - 0.98556445e-3_f64 * t344 * t3309 - 4.0_f64 * t3312 - 4.0_f64 * t278 * t3038;
    (t3309, t3316)
}
