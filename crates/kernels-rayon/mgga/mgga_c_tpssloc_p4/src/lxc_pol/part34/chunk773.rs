//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 773/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk773(t13908: f64, t973: f64, t10508: f64, t1616: f64, t248: f64, t1020: f64, t3069: f64, t4669: f64, t1612: f64, t3082: f64, t1606: f64, t698: f64) -> (f64, f64, f64, f64, f64) {
    let t13909 = t973 * t13908;
    let t13965 = t248 * t10508 * t1616;
    let t13966 = t1020 * t13965;
    let t13995 = t4669 * t3069;
    let t14117 = t1612 * t3082;
    let t14159 = t698 * t1606;
    (t13909, t13966, t13995, t14117, t14159)
}
