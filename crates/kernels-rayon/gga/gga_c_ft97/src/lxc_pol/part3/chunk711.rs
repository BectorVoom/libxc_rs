//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 711/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk711(t1775: f64, t3927: f64, t1609: f64, t2378: f64, t2427: f64, t6: f64, t224: f64, t1113: f64, t695: f64, t3758: f64, t122: f64, t677: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13388 = 2.0_f64 / 9.0_f64 * t1775 * t3927;
    let t13411 = t1609 * t2378;
    let t13442 = t2427 * t6;
    let t13443 = t224 * t13442;
    let t13463 = t695 * t1113;
    let t13464 = t3758 * t13463;
    let t13467 = t695 * t122;
    let t13468 = t677 * t13467;
    (t13388, t13411, t13443, t13463, t13464, t13468)
}
