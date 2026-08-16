//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1625/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1625(t3450: f64, t3966: f64, t3449: f64, t14749: f64, t4908: f64, t3448: f64, t4928: f64, t3451: f64, t11579: f64, t4919: f64, t11584: f64, t1174: f64, t15294: f64, t15300: f64, t15304: f64, t15307: f64, t3443: f64, t3447: f64, t3457: f64, t3461: f64, t4889: f64) -> f64 {
    let t15313 = t3450 * t3966;
    let t15314 = t3449 * t15313;
    let t15317 = t4908 * t14749;
    let t15320 = t3448 * t4928;
    let t15321 = t15320 * t3451;
    let t15324 = t4919 * t11579;
    let t15327 = t4919 * t11584;
    let t15330 = 0.11111111111111111111e-2_f64 * t3447 * t15294 - 0.98765432098765432097e-3_f64 * t4889 * t3443 + 0.6172839506172839506e-4_f64 * t15300 - 0.83333333333333333332e-3_f64 * t1174 * t15304 + 0.49382716049382716048e-3_f64 * t15307 + 0.74074074074074074073e-3_f64 * t4889 * t3461 + 0.14814814814814814815e-2_f64 * t4889 * t3457 + 0.55555555555555555554e-3_f64 * t3447 * t15314 - 0.11111111111111111111e-2_f64 * t3447 * t15317 + 0.55555555555555555554e-3_f64 * t3447 * t15321 + 0.27777777777777777777e-3_f64 * t3447 * t15324 + 0.55555555555555555554e-3_f64 * t3447 * t15327;
    t15330
}
