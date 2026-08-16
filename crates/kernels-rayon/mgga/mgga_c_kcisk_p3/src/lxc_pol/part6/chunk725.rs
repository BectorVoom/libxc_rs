//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 725/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk725(t12974: f64, t311: f64, t313: f64, t3841: f64, t306: f64, t315: f64, t1170: f64, t3675: f64, t305: f64, t320: f64, t3678: f64, t330: f64, t3721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12975 = 28.0_f64 / 27.0_f64 * t12974;
    let t12998 = t311 * t3841 * t313;
    let t12999 = 0.36514074074074074075e0_f64 * t12998;
    let t13000 = 0.93011851851851851854e0_f64 * t12974;
    let t13009 = 1.0_f64 / t306 / t315 / 4.0_f64;
    let t13020 = 1.0_f64 / t3675 / t1170;
    let t13021 = t305 * t13020;
    let t13023 = 1.0_f64 / t3678 / t320;
    let t13027 = 0.28842592592592592592e-1_f64 * t12974;
    let t13064 = 1.0_f64 / t3721 / t330;
    (t12975, t12998, t12999, t13000, t13009, t13021, t13023, t13027, t13064)
}
