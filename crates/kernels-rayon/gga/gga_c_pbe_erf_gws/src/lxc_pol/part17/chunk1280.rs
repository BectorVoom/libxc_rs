//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1280/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1280(t3209: f64, t51682: f64, t14121: f64, t8761: f64, t8806: f64, t13917: f64, t14424: f64, t9371: f64, t51898: f64, t9243: f64, t1105: f64, t12213: f64, t13994: f64, t14106: f64, t14627: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t4385: f64, t51719: f64, t51724: f64, t51726: f64, t51745: f64, t53790: f64, t53795: f64, t53804: f64, t6781: f64, t6793: f64) -> f64 {
    let t53806 = t51682 * t3209;
    let t53807 = 7.0_f64 / 24.0_f64 * t53806;
    let t53809 = t14121 * t8761;
    let t53811 = t14121 * t8806;
    let t53816 = t13917 * t14424 * t9371;
    let t53832 = t51898 * t9243;
    let t53834 = -t4385 * t53790 / 48.0_f64 - t6793 * t53795 / 8.0_f64 + t53804 / 768.0_f64 - t53807 + 7.0_f64 / 288.0_f64 * t51719 + t53809 / 16.0_f64 + t53811 / 8.0_f64 - 7.0_f64 / 144.0_f64 * t51724 - 7.0_f64 / 72.0_f64 * t51726 - t53816 / 768.0_f64 + t3066 * t2409 * t12213 * t13994 / 24.0_f64 + t2408 * t2409 * t6781 * t14627 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t51745 + t2408 * t2409 * t2376 * t14106 * t1105 / 48.0_f64 - t53832 / 4.0_f64;
    t53834
}
