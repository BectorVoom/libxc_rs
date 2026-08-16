//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 743/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk743(t1001: f64, t8419: f64, t8483: f64, t8487: f64, t8490: f64, t8494: f64, t8496: f64, t8498: f64, t8502: f64, t8506: f64, t8512: f64, t8515: f64) -> (f64, f64) {
    let t8517 = t8419 * t1001;
    let t8519 = 0.16221005325193686047e-3_f64 * t8483 - 0.20855578275249024918e-2_f64 * t8487 - 0.17714874716515957771e-4_f64 * t8490 + 0.29524791194193262952e-5_f64 * t8494 - 0.28840947468194373793e-3_f64 * t8496 + 0.12360406057797588768e-3_f64 * t8498 + 0.772525378612349298e-5_f64 * t8502 - 0.45785004105758568397e-6_f64 * t8506 - 0.27721444647547803303e-5_f64 * t8512 + 0.6951859425083008306e-4_f64 * t8515 - 0.26319242435966565832e-3_f64 * t8517;
    (t8517, t8519)
}
