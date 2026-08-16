//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1103/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1103(t1356: f64, t1668: f64, t36992: f64, t41932: f64, t42196: f64, t42201: f64, t42205: f64, t42207: f64, t42217: f64, t46589: f64, t46867: f64, t48000: f64, t48009: f64, t48011: f64, t48014: f64, t48017: f64, t48022: f64, t4965: f64, t530: f64, t7703: f64, t884: f64, t8876: f64, t9960: f64) -> f64 {
    let t48026 = -0.25538759935978703639e-4_f64 * t48000 - 0.23948483403727617128e0_f64 * t1356 * t7703 * t46867 - 0.4726e1_f64 * t1668 * t8876 - 0.4726e1_f64 * t530 * t41932 + 0.13637330827122670864e0_f64 * t48009 + 0.27274661654245341728e-1_f64 * t48011 - 0.26668558061928778579e0_f64 * t42196 + 0.44903406381989282115e-1_f64 * t48014 + t36992 - 0.72732431077987577944e-1_f64 * t42201 - 0.5987120850931904282e-1_f64 * t48017 + t42205 - t42207 + 0.59590439850616975157e-4_f64 * t42217 + 0.39914139006212695214e-1_f64 * t4965 * t9960 + 0.79828278012425390427e-1_f64 * t48022 + 0.59871208509319042821e-1_f64 * t884 * t46589;
    t48026
}
