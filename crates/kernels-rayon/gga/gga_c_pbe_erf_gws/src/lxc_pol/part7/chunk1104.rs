//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1104/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1104(t19756: f64, t2417: f64, t353: f64, t4386: f64, t19615: f64, t814: f64, t859: f64, t19608: f64, t19714: f64, t19722: f64, t19726: f64, t19728: f64, t19731: f64, t19735: f64, t19738: f64, t19745: f64, t19751: f64, t2118: f64, t2362: f64, t2382: f64, t2388: f64, t2397: f64, t3074: f64, t3079: f64, t328: f64, t4395: f64, t6112: f64, t6135: f64, t6158: f64, t6793: f64, t6802: f64, t822: f64, t833: f64) -> f64 {
    let t19759 = t4386 * t353 * t19756 * t2417;
    let t19764 = t859 * t353 * t19615 * t814;
    let t19767 = 7.0_f64 / 48.0_f64 * t3074 * t2118 * t19714 * t328 * t3079 + t6802 * t2397 / 24.0_f64 - 7.0_f64 / 24.0_f64 * t19722 - t2388 * t6135 / 4.0_f64 + 7.0_f64 / 24.0_f64 * t19726 - 7.0_f64 / 4.0_f64 * t19728 - 7.0_f64 / 36.0_f64 * t19731 + 35.0_f64 / 72.0_f64 * t19735 + t822 * t19738 * t833 / 96.0_f64 + t6112 * t2397 / 24.0_f64 - 7.0_f64 / 48.0_f64 * t3074 * t4395 * t19745 * t2362 - 7.0_f64 / 48.0_f64 * t2382 * t6158 * t19751 * t2362 - t6793 * t19759 / 2.0_f64 + t19608 * t19764 / 16.0_f64;
    t19767
}
