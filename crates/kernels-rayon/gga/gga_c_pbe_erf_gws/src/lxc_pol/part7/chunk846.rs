//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 846/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk846(t4806: f64, t4814: f64, t4687: f64, t4710: f64, t4713: f64, t4717: f64, t4825: f64, t4810: f64, t4817: f64, t4819: f64, t4821: f64, t4823: f64, t4827: f64, t4830: f64, t4833: f64, t6850: f64, t6856: f64) -> f64 {
    let t16366 = 0.14035736153892489771e2_f64 * t4806;
    let t16368 = 0.22787712934626154593e-2_f64 * t4814;
    let t16369 = 0.4274e0_f64 * t4687;
    let t16370 = 0.28493333333333333333e0_f64 * t4710;
    let t16371 = 0.2137e0_f64 * t4713;
    let t16372 = 0.34366858576436911004e1_f64 * t4717;
    let t16379 = 240.0_f64 * t4825;
    let t16383 = t16366 + 0.29298488058805055905e-2_f64 * t4810 - t16368 + t16369 + t16370 - t16371 - t16372 - 0.21973866044103791929e-2_f64 * t4817 + 36.0_f64 * t6850 + 8.0_f64 * t6856 + 96.0_f64 * t4819 - 96.0_f64 * t4821 + 48.0_f64 * t4823 + t16379 - 384.0_f64 * t4827 + 240.0_f64 * t4830 + 4.0_f64 * t4833;
    t16383
}
