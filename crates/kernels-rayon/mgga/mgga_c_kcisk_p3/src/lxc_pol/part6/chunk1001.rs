//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1001/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1001(t30503: f64, t395: f64, t2075: f64, t8048: f64, t13472: f64, t1313: f64, t30158: f64, t1312: f64, t6205: f64, t8054: f64, t6204: f64, t1309: f64, t20128: f64, t20255: f64, t2164: f64, t25981: f64, t25985: f64, t26008: f64, t26065: f64, t26075: f64, t26086: f64, t30270: f64, t30274: f64, t30278: f64, t3935: f64, t405: f64, t6157: f64, t8033: f64, t8037: f64, t8041: f64, t8045: f64, sigma0: f64) -> f64 {
    let t30504 = t30503 * sigma0;
    let t30505 = t30504 * t395;
    let t30510 = t2075 * t8048;
    let t30511 = t13472 * t30510;
    let t30514 = t1313 * t30158;
    let t30515 = t1312 * t30514;
    let t30522 = t6205 * t8054;
    let t30523 = t6204 * t30522;
    let t30534 = 0.10794473229706390328e0_f64 * t3935 * t30270 - 0.10794473229706390328e0_f64 * t3935 * t30274 - 0.53972366148531951639e-1_f64 * t3935 * t30278 + 0.17990788716177317213e-1_f64 * t25981 + 0.35981577432354634425e-1_f64 * t25985 + 0.5397236614853195164e-1_f64 * t30505 * t405 - 0.10794473229706390328e0_f64 * t20255 * t8037 + 0.10794473229706390328e0_f64 * t3935 * t30511 + 0.17990788716177317213e-1_f64 * t1309 * t30515 - 0.10794473229706390328e0_f64 * t6157 * t8041 + 0.53972366148531951639e-1_f64 * t26008 * t2164 + 0.32383419689119170984e0_f64 * t1309 * t30523 + 0.53972366148531951639e-1_f64 * t6157 * t8045 + 0.71963154864709268852e-1_f64 * t6157 * t8033 - 0.35981577432354634425e-1_f64 * t26065 - 0.11993859144118211475e-1_f64 * t20128 - 0.35981577432354634425e-1_f64 * t26075 + 0.2398771828823642295e-1_f64 * t26086;
    t30534
}
