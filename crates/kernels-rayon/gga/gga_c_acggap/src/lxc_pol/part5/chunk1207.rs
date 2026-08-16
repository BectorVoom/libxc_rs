//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1207/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1207(t1181: f64, t1350: f64, t1567: f64, t3361: f64, t4396: f64, t5936: f64, t1426: f64, t1713: f64, t175: f64, t384: f64, t879: f64, t13298: f64, t13364: f64, t21143: f64, t525: f64) -> (f64, f64, f64, f64) {
    let t22021 = t3361 * t1181 * t1567 * t1350;
    let t22023 = t4396 * t5936;
    let t22032 = t384 * t1426 * t175 * t1713 * t879;
    let t22038 = t13298 * t13364 * t525 * t21143;
    (t22021, t22023, t22032, t22038)
}
