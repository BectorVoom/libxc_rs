//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 875/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk875(t1866: f64, t2635: f64, t1885: f64, t1820: f64, t1019: f64, t1775: f64, t1006: f64, t1806: f64, t1062: f64, t1903: f64, t7380: f64, t4940: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t7371: f64, t7374: f64, t7376: f64, t7378: f64, t7383: f64, t7386: f64, t7389: f64, t7392: f64, t7395: f64, t7398: f64, t7401: f64) -> (f64, f64, f64, f64, f64) {
    let t7533 = t2635 * t1866;
    let t7534 = t1885 * t7533;
    let t7536 = 4.0_f64 / 15.0_f64 * t1820 * t7534;
    let t7538 = 2.0_f64 / 15.0_f64 * t1775 * t1019;
    let t7540 = 4.0_f64 / 15.0_f64 * t1006 * t1806;
    let t7541 = t1062 * t1903;
    let t7549 = 0.2518888888888888889e-2_f64 * t7380;
    let t7559 = t4940 + 0.16792592592592592593e-2_f64 * t4941 - 0.41981481481481481482e-3_f64 * t4943 + 0.12594444444444444445e-2_f64 * t4945 - 0.62972222222222222223e-3_f64 * t4947 + 0.83962962962962962964e-3_f64 * t7374 - 0.83962962962962962965e-3_f64 * t7378 + t7549 - 0.1385388888888888889e-1_f64 * t7376 + 0.20990740740740740742e-2_f64 * t7383 - 0.75566666666666666669e-2_f64 * t7386 + 0.50377777777777777779e-2_f64 * t7389 + 0.12594444444444444445e-2_f64 * t7392 + 0.11335e-1_f64 * t7395 - 0.15113333333333333334e-1_f64 * t7398 - 0.37783333333333333334e-2_f64 * t7401 + 0.37783333333333333334e-2_f64 * t7371;
    (t7536, t7538, t7540, t7541, t7559)
}
