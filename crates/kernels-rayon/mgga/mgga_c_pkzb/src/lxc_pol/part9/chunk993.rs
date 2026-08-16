//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 993/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk993(t24: f64, t7930: f64, t1003: f64, t6097: f64, t2179: f64, t8: f64, t1429: f64, t821: f64, t1652: f64, t1655: f64, t3019: f64, t3022: f64, t6786: f64, t82: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t7931 = 0.59793333333333333334e0_f64 * t7930;
    let t7932 = t6097 * t1003;
    let t7935 = t2179 * t8;
    let t7940 = t821 * t1429;
    let t7945 = piecewise3(t90, 0.0_f64, -28.0_f64 / 27.0_f64 * t7932 * t1652 - 16.0_f64 / 9.0_f64 * t7935 * t6786 + 4.0_f64 / 9.0_f64 * t3019 * t1655 + 2.0_f64 / 3.0_f64 * t7940 - 2.0_f64 * t3022 * t82);
    (t7931, t7932, t7935, t7940, t7945)
}
