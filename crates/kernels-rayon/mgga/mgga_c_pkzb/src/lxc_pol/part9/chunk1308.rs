//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1308/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1308(t394: f64, t6506: f64, t178: f64, t19080: f64, t19079: f64, t1227: f64, t17938: f64, t2370: f64, t6460: f64, t19091: f64, t2401: f64, t3185: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22966 = t6506 * t394;
    let t22971 = t19080 * t178;
    let t22972 = t19079 * t22971;
    let t22973 = t1227 * t17938;
    let t22974 = t6460 * t2370;
    let t22979 = t19091 * t22971;
    let t22980 = t6460 * t394;
    let t22988 = t3185 * t2401 * t3188;
    (t22966, t22971, t22972, t22973, t22974, t22979, t22980, t22988)
}
