//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 954/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk954(t18740: f64, t684: f64, t2606: f64, t5134: f64, t681: f64, t89: f64, t1168: f64, t3972: f64, t2568: f64, t242: f64, t10134: f64, t14240: f64, t14281: f64, t14283: f64, t18709: f64, t18714: f64, t18718: f64, t18721: f64, t18726: f64, t18731: f64, t18734: f64, t18737: f64, t1901: f64, t446: f64) -> (f64, f64) {
    let t18741 = t18740 * t684;
    let t18742 = t2606 * t18741;
    let t18746 = t89 * t681 * t5134;
    let t18749 = t1168 * t3972;
    let t18750 = t2568 * t18749;
    let t18751 = t242 * t18750;
    let t18754 = t1901 * t18709 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t18714 - 2.0_f64 / 27.0_f64 * t1901 * t18718 - t14240 + 2.0_f64 / 9.0_f64 * t1901 * t18721 + 2.0_f64 / 9.0_f64 * t1901 * t18726 + t1901 * t18731 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t18734 + 2.0_f64 / 9.0_f64 * t1901 * t18737 + t1901 * t18742 / 9.0_f64 - t18746 / 9.0_f64 + t14281 + t14283 - 4.0_f64 / 81.0_f64 * t10134 + 4.0_f64 / 3.0_f64 * t446 * t18751;
    (t18750, t18754)
}
