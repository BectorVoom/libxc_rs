//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 953/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk953(t1691: f64, t408: f64, t32129: f64, t5608: f64, t136815: f64, t373: f64, t32238: f64, t7837: f64, t32339: f64, t376: f64, t89: f64, t32343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t137028 = t408 * t1691;
    let t137035 = t32129 * t5608;
    let t137037 = t136815 * t373;
    let t137047 = t7837 * t32238;
    let t137070 = t89 * t376 * t32339;
    let t137073 = t89 * t376 * t32343;
    (t137028, t137035, t137037, t137047, t137070, t137073)
}
