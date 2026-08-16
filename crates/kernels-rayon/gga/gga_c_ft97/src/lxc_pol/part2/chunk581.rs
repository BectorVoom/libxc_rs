//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 581/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk581(t1168: f64, t761: f64, t684: f64, t2606: f64, t1901: f64, t2549: f64, t2553: f64, t2554: f64, t2556: f64, t2584: f64, t3281: f64, t3835: f64, t3839: f64, t3844: f64, t3848: f64, t3852: f64, t3856: f64, t3861: f64, t3866: f64, t446: f64) -> (f64, f64, f64, f64) {
    let t3869 = t761 * t1168;
    let t3870 = t3869 * t684;
    let t3871 = t2606 * t3870;
    let t3874 = t2584 / 27.0_f64 + t2554 / 9.0_f64 + t2556 / 9.0_f64 + t2553 - t2549 / 9.0_f64 + t3835 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t3839 + t446 * t3844 / 3.0_f64 - t446 * t3848 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t3281 * t3852 - t446 * t3856 / 9.0_f64 + t446 * t3861 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t3866 + t1901 * t3871 / 9.0_f64;
    (t3869, t3870, t3871, t3874)
}
