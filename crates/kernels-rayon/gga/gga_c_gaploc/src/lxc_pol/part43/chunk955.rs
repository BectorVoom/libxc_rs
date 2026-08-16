//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 955/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk955(t1365: f64, t38272: f64, t6525: f64, t2268: f64, t426: f64, t46849: f64, t535: f64, t13740: f64, t484: f64, t11977: f64, t6763: f64, t1063: f64, t6750: f64) -> (f64, f64, f64, f64, f64) {
    let t47036 = t6525 * t1365 * t38272;
    let t47040 = t2268 * t535 * t46849 * t426;
    let t47042 = t484 * t13740;
    let t47047 = t2268 * t11977 * t6763;
    let t47050 = t1063 * t11977 * t6750;
    (t47036, t47040, t47042, t47047, t47050)
}
