//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 801/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk801(t1775: f64, t4519: f64, t4523: f64, t4512: f64, t363: f64, t4606: f64, t11756: f64, t1800: f64, t358: f64, t432: f64, t11762: f64, t11718: f64, t11720: f64, t11732: f64, t11734: f64, t11745: f64, t11755: f64, t11761: f64, t16418: f64, t16421: f64, t16424: f64, t16427: f64, t16430: f64, t16433: f64, t16439: f64, t3139: f64, t462: f64, t8301: f64, t8302: f64) -> f64 {
    let t16442 = t1775 * t4519;
    let t16444 = t1775 * t4523;
    let t16446 = t1775 * t4512;
    let t16448 = t4606 * t363;
    let t16449 = t11756 * t16448;
    let t16452 = t1800 * t358;
    let t16454 = t16452 * t4606 * t432;
    let t16457 = t11762 * t16448;
    let t16461 = 8.0_f64 / 3.0_f64 * t3139 * t16418 + 4.0_f64 / 3.0_f64 * t462 * t16421 - 10.0_f64 / 27.0_f64 * t462 * t16424 - 8.0_f64 / 9.0_f64 * t3139 * t16427 + 2.0_f64 / 3.0_f64 * t462 * t16430 + 2.0_f64 / 9.0_f64 * t462 * t16433 + 4.0_f64 / 9.0_f64 * t11718 - 8.0_f64 / 27.0_f64 * t11720 - t11732 + 4.0_f64 * t462 * t16439 - 2.0_f64 / 9.0_f64 * t16442 + t16444 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t16446 + 4.0_f64 / 9.0_f64 * t11755 * t16449 - 4.0_f64 / 3.0_f64 * t11761 * t16454 - 4.0_f64 / 3.0_f64 * t11761 * t16457 - t8301 - t11734 - t11745 - 4.0_f64 / 9.0_f64 * t8302;
    t16461
}
