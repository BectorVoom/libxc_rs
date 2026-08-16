//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 792/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk792(t36541: f64, t7473: f64, t1969: f64, t34846: f64, t2001: f64, t2002: f64, t305: f64, t321: f64, t7345: f64, t7927: f64, t35207: f64, t7354: f64) -> (f64, f64, f64, f64, f64) {
    let t36769 = t36541 * t7473;
    let t36772 = t34846 * t1969;
    let t36787 = t2001 * t305 * t2002 * t321;
    let t36796 = t7345 * t7927;
    let t36797 = 0.12195059916630011326e-2_f64 * t36796;
    let t36801 = t35207 * t7354;
    (t36769, t36772, t36787, t36797, t36801)
}
