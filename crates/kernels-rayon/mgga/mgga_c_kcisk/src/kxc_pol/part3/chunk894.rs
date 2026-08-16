//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 894/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk894(t3512: f64, t3765: f64, t1339: f64, t1440: f64, t3777: f64, t3776: f64, t1415: f64, t1411: f64, t12817: f64, t1451: f64, t1450: f64, t3795: f64) -> (f64, f64, f64, f64, f64) {
    let t13366 = t3512 * t3765;
    let t13367 = t1339 * t13366;
    let t13369 = t3777 * t1440;
    let t13370 = t3776 * t13369;
    let t13371 = t1415 * t13370;
    let t13372 = t1411 * t13371;
    let t13374 = t12817 * t1451;
    let t13375 = t1411 * t13374;
    let t13377 = t3795 * t1450;
    (t13367, t13369, t13372, t13375, t13377)
}
