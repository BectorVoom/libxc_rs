//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 941/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk941(t18532: f64, t2607: f64, t2606: f64, t1882: f64, t5087: f64, t5083: f64, t5079: f64, t5075: f64, t17720: f64, t17724: f64, t17729: f64, t17734: f64, t17738: f64, t17742: f64, t17746: f64, t17751: f64, t17755: f64, t17759: f64, t17763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18533 = t2607 * t18532;
    let t18534 = t2606 * t18533;
    let t18538 = t1882 * t5087;
    let t18540 = t1882 * t5083;
    let t18542 = t1882 * t5079;
    let t18544 = t1882 * t5075;
    let t18557 = -2.0_f64 / 27.0_f64 * t17720 + t17724 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t17729 - 2.0_f64 / 27.0_f64 * t17734 - 4.0_f64 / 9.0_f64 * t17738 - 2.0_f64 / 9.0_f64 * t17742 - 2.0_f64 / 3.0_f64 * t17746 - 10.0_f64 / 81.0_f64 * t17751 + 8.0_f64 / 27.0_f64 * t17755 + 2.0_f64 / 9.0_f64 * t17759 + 2.0_f64 / 27.0_f64 * t17763;
    (t18534, t18538, t18540, t18542, t18544, t18557)
}
