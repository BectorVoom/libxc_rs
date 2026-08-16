//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1021/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1021(t10482: f64, t21390: f64, t1021: f64, t248: f64, t3131: f64, t360: f64, t10278: f64, t20234: f64, t2979: f64, t21122: f64, t4510: f64, t13769: f64, t17863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21391 = t21390 * t10482;
    let t21393 = t248 * t1021 * t21391;
    let t21396 = t21390 * t3131;
    let t21398 = t248 * t1021 * t21396;
    let t21403 = t21390 * t360;
    let t21405 = t248 * t1021 * t21403;
    let t21409 = t10278 * t20234;
    let t21410 = t2979 * t21409;
    let t21413 = t4510 * t21122;
    let t21416 = t13769 * t17863;
    (t21391, t21393, t21396, t21398, t21403, t21405, t21409, t21410, t21413, t21416)
}
