//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1006/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1006(t495: f64, t9369: f64, t1057: f64, t3649: f64, t1523: f64, t2813: f64, t462: f64, t1052: f64, t3647: f64, t7276: f64, t7310: f64, t7312: f64, t7314: f64, t7484: f64, t7487: f64, t7490: f64, t7493: f64, t8949: f64, t9314: f64, t9338: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9370 = t9369 * t495;
    let t9372 = t1057 * t3649;
    let t9374 = t1523 * t2813;
    let t9375 = t462 * t9374;
    let t9376 = t1052 * t3649;
    let t9379 = 8.0_f64 * t1052 * t3647;
    let t9381 = 8.0_f64 * t1057 * t3647;
    let t9383 = t462 * t9370 - 24.0_f64 * t7276 + t7310 + t7312 + 2.0_f64 * t7314 + t7484 - t7487 - t7490 + t7493 - t8949 - t9314 - t9338 - 8.0_f64 * t9372 + t9375 + 8.0_f64 * t9376 + t9379 - t9381;
    (t9370, t9372, t9374, t9375, t9379, t9381, t9383)
}
