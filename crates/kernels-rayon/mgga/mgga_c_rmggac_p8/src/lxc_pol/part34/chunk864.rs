//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 864/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk864(t3352: f64, t69004: f64, t8416: f64, t3351: f64, t515: f64, t8982: f64, t9188: f64, t2144: f64, t8985: f64, t875: f64, t8976: f64, t1986: f64, t2396: f64) -> (f64, f64, f64, f64, f64) {
    let t75465 = t69004 * t3352 * t8416;
    let t75469 = t3351 * t9188 * t515 * t8982;
    let t75473 = t3351 * t3352 * t2144 * t8985;
    let t75477 = t3351 * t3352 * t875 * t8976;
    let t75479 = t1986 * t2396;
    (t75465, t75469, t75473, t75477, t75479)
}
