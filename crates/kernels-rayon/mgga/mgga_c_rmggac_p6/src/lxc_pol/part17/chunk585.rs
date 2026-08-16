//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 585/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk585(t678: f64, t7939: f64, t2153: f64, t275: f64, t1347: f64, t669: f64, t2416: f64, t7487: f64, t2160: f64, t2339: f64, t638: f64, t2323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7940 = t7939 * t678;
    let t7941 = 0.19863479950205658386e-4_f64 * t7940;
    let t7947 = t275 * t2153;
    let t7948 = 2.0_f64 * t7947;
    let t7949 = t1347 * t669;
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    (t7941, t7948, t7949, t8328, t8331, t8334)
}
