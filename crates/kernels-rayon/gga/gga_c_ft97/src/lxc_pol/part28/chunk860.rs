//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 860/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk860(t1871: f64, t7165: f64, t986: f64, t1339: f64, t452: f64, t6454: f64, t26166: f64, t6547: f64, t11490: f64, t34368: f64, t83: f64, t34544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34718 = t1871 * t986 * t7165;
    let t34722 = t452 * t1339 * t6454;
    let t34725 = t26166 * t6547;
    let t34726 = t11490 * t34725;
    let t34729 = t83 * t34368;
    let t34732 = t83 * t34544;
    (t34718, t34722, t34725, t34726, t34729, t34732)
}
