//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1983/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1983(t2047: f64, t5611: f64, t5584: f64, t101698: f64, t13176: f64, t16935: f64, t2617: f64, t26608: f64, t26656: f64, t29010: f64, t4166: f64, t4182: f64, t4234: f64, t4281: f64, t4291: f64, t7837: f64, t829: f64, t85003: f64, t87635: f64, t87653: f64, t87666: f64, t92760: f64, t92768: f64, t92795: f64, t98575: f64) -> (f64, f64) {
    let t101708 = t2047 * t5611;
    let t101715 = t2047 * t5584;
    let t101734 = -0.6579736267392905746e-1_f64 * t98575 - t92760 + 2.0_f64 * t4281 * t101708 * t4182 + t92768 - 2.0_f64 * t4291 * t26656 * t4234 - t4291 * t101715 * t829 - 0.5117572652416704469e0_f64 * t87635 - t4291 * t101708 * t829 - 0.3289868133696452873e-1_f64 * t87653 + t85003 - t2617 * t29010 + 4.0_f64 * t4281 * t26656 * t16935 - 2.0_f64 * t13176 * t7837 - 2.0_f64 * t4166 * t26608 + 4.0_f64 * t4281 * t101698 * t4182 - 0.25587863262083522345e0_f64 * t87666 + t92795;
    (t101715, t101734)
}
