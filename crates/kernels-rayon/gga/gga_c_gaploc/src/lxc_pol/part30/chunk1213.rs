//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1213/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1213(t32214: f64, t7290: f64, t2530: f64, t8469: f64, t2508: f64, t2580: f64, t24339: f64, t935: f64, t10782: f64, t1865: f64, t11004: f64, t21556: f64, t3420: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32215 = t7290 * t32214;
    let t32219 = t8469 * t2530;
    let t32222 = 0.30762104920568897134e-1_f64 * t2508 * t2580 * t32219;
    let t32223 = t24339 * t935;
    let t32226 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t32223;
    let t32230 = t10782 * t1865;
    let t32234 = t11004 * t1865;
    let t32241 = 0.20508069947045931424e-1_f64 * t21556 * t3420;
    (t32215, t32219, t32222, t32223, t32226, t32230, t32234, t32241)
}
