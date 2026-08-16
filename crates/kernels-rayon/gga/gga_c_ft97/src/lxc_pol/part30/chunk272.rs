//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 272/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk272(t1614: f64, t213: f64, t1109: f64, t3762: f64, t709: f64, t1701: f64, t1127: f64, t25: f64, t224: f64, t226: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3775 = t1614 * t213;
    let t3776 = t3775 * t1109;
    let t3777 = t3776 * t3762;
    let t3780 = t213 * t1109;
    let t3781 = t3780 * t709;
    let t3782 = t1701 * t3781;
    let t3785 = t1127 * t25;
    let t3786 = t3785 * t3762;
    let t3789 = t224 * t226;
    (t3777, t3780, t3781, t3782, t3786, t3789)
}
