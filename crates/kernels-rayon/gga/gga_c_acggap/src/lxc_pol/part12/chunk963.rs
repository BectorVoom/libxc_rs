//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 963/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk963(t1177: f64, t31824: f64, t13364: f64, t31115: f64, t31116: f64, t30049: f64, t7461: f64, t1089: f64, t1198: f64, t2079: f64, t2080: f64, t1967: f64, t7523: f64) -> (f64, f64, f64, f64, f64) {
    let t31825 = t31824 * t1177;
    let t31832 = t31115 * t13364 * t31116;
    let t31839 = t30049 * t7461;
    let t31843 = t2079 * t1089 * t1198 * t2080;
    let t31845 = t1967 * t7523;
    (t31825, t31832, t31839, t31843, t31845)
}
