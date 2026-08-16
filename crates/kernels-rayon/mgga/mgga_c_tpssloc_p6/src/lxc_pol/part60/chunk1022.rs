//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1022/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1022(t1983: f64, t33335: f64, t5161: f64, t33366: f64, t7685: f64, t5450: f64, t8595: f64, t2075: f64, t28017: f64, t652: f64, t5493: f64, t33620: f64, t4028: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128441 = 2.0_f64 * t1983 * t33335 * t5161;
    let t128443 = 2.0_f64 * t7685 * t33366;
    let t128444 = t5450 * t8595;
    let t128449 = 2.0_f64 * t652 * t2075 * t28017;
    let t128452 = 2.0_f64 * t652 * t8595 * t5493;
    let t128454 = 4.0_f64 * t4028 * t33620;
    (t128441, t128443, t128444, t128449, t128452, t128454)
}
