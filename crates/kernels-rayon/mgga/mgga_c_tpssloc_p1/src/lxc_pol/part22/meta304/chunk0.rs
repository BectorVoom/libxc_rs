//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1470/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1470(t10704: f64, t1556: f64, t13566: f64, t13602: f64, t10813: f64, t1568: f64, t2932: f64, t4471: f64, t300: f64, t4446: f64, t3053: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14395 = t1556 * t10704;
    let t14409 = 0.2283111111111111111e-1_f64 * t13566;
    let t14410 = 0.11415555555555555555e-1_f64 * t13602;
    let t14442 = t1568 * t10813;
    let t14459 = t4471 * t2932;
    let t14473 = t300 * t4446;
    let t14495 = t4644 * t3053 / 3456.0_f64;
    (t14395, t14409, t14410, t14442, t14459, t14473, t14495)
}
