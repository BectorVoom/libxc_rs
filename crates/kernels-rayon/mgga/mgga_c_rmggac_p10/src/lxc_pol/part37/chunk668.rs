//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 668/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk668(t14180: f64, t34884: f64, t7323: f64, t7557: f64, t14089: f64, t14090: f64, t49: f64, t2051: f64, t388: f64, t14082: f64, t20925: f64, t253: f64, t34747: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68399 = t34884 * t14180;
    let t68401 = t7323 * t7557;
    let t68406 = t14089 * t14090 * t49;
    let t68407 = t388 * t2051;
    let t68408 = t68406 * t68407;
    let t68409 = 0.13469175824740901074e-6_f64 * t68408;
    let t68414 = t253 * t34747 * t14082 * t20925 * t2051;
    (t68399, t68401, t68406, t68407, t68409, t68414)
}
