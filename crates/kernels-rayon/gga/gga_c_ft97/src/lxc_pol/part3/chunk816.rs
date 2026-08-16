//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 816/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk816(t16686: f64, t363: f64, t9073: f64, t446: f64, t15756: f64, t569: f64, t3281: f64, t4462: f64, t558: f64, t1969: f64, t15768: f64, t12307: f64, t12309: f64, t12311: f64, t12328: f64, t12357: f64, t12359: f64, t12366: f64, t12913: f64, t16668: f64, t16673: f64, t16677: f64, t16679: f64, t16684: f64, t8796: f64, t9065: f64, t9072: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16687 = t16686 * t363;
    let t16688 = t9073 * t16687;
    let t16689 = t446 * t16688;
    let t16691 = t569 * t15756;
    let t16692 = t3281 * t16691;
    let t16694 = t4462 * t558;
    let t16695 = t1969 * t16694;
    let t16696 = t446 * t16695;
    let t16698 = t569 * t15768;
    let t16699 = t446 * t16698;
    let t16704 = -2.0_f64 / 9.0_f64 * t16668 - 2.0_f64 / 9.0_f64 * t16673 + 2.0_f64 / 27.0_f64 * t16677 - t16679 / 27.0_f64 + t16684 / 18.0_f64 - t16689 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t16692 + t16696 / 18.0_f64 + t16699 / 9.0_f64 - t12307 - t12309 + t12311 - t12328 - 2.0_f64 / 27.0_f64 * t9065 - 2.0_f64 / 81.0_f64 * t8796 - t12357 + 2.0_f64 / 27.0_f64 * t12359 - t12913 - t9072 + t12366;
    (t16687, t16689, t16692, t16694, t16696, t16699, t16704)
}
