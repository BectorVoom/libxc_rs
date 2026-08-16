//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2233/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2233(t18427: f64, t3449: f64, t18221: f64, t4908: f64, t15320: f64, t4904: f64, t15313: f64, t4919: f64, t11531: f64, t15265: f64, t15376: f64, t18404: f64, t18410: f64, t18413: f64, t18417: f64, t18421: f64, t18424: f64, t3447: f64, t4901: f64) -> f64 {
    let t18428 = t3449 * t18427;
    let t18431 = t4908 * t18221;
    let t18434 = t15320 * t4904;
    let t18437 = t4919 * t15313;
    let t18442 = 0.37037037037037037036e-3_f64 * t3447 * t18404 - 0.19753086419753086419e-2_f64 * t15376 * t4901 + 0.27777777777777777777e-3_f64 * t3447 * t18410 - 0.55555555555555555554e-3_f64 * t3447 * t18413 + 0.27777777777777777777e-3_f64 * t3447 * t18417 + 0.27777777777777777777e-3_f64 * t3447 * t18421 - 0.11111111111111111111e-2_f64 * t3447 * t18424 + 0.55555555555555555554e-3_f64 * t3447 * t18428 - 0.16666666666666666666e-2_f64 * t3447 * t18431 + 0.55555555555555555554e-3_f64 * t3447 * t18434 + 0.55555555555555555554e-3_f64 * t3447 * t18437 + 0.6172839506172839506e-4_f64 * t11531 + 0.98765432098765432093e-3_f64 * t15265;
    t18442
}
