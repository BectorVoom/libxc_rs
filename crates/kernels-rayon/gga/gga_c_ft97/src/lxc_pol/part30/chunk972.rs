//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 972/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk972(t231: f64, t33829: f64, t33840: f64, t6308: f64, t681: f64, t1486: f64, t33852: f64, t33954: f64, t2347: f64, t7611: f64, t33855: f64, t2360: f64, t7584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t143163 = t231 * t33829;
    let t143177 = t6308 * t681 * t33840;
    let t143180 = t1486 * t681 * t33852;
    let t143187 = t1486 * t681 * t33954;
    let t143193 = t7611 * t2347;
    let t143204 = t1486 * t681 * t33855;
    let t143217 = t7584 * t2360;
    (t143163, t143177, t143180, t143187, t143193, t143204, t143217)
}
