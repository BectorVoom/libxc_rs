//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 487/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk487(t1131: f64, t668: f64, t505: f64, t2354: f64, t446: f64, t2371: f64, t713: f64, t193: f64, t89: f64, t2382: f64, t688: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3712 = t1131 * t668;
    let t3713 = t3712 * t505;
    let t3714 = t2354 * t3713;
    let t3715 = t446 * t3714;
    let t3717 = t2371 * t1131;
    let t3718 = t3717 * t713;
    let t3720 = t89 * t193 * t3718;
    let t3722 = t688 * t2382;
    let t3723 = t2379 * t3722;
    (t3713, t3714, t3715, t3717, t3718, t3720, t3723)
}
