//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1853/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1853(t1044: f64, t13611: f64, t248: f64, t1023: f64, t13975: f64, t4582: f64, t3121: f64, t4593: f64, t3041: f64, t1031: f64, t4616: f64, t1612: f64, t3082: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14093 = t248 * t1044 * t13611;
    let t14098 = t13975 * t1023;
    let t14099 = t4582 * t14098;
    let t14102 = t4593 * t3121;
    let t14103 = t4582 * t14102;
    let t14106 = t4593 * t3041;
    let t14107 = t4582 * t14106;
    let t14114 = t4616 * t1031;
    let t14117 = t1612 * t3082;
    (t14093, t14098, t14099, t14102, t14103, t14106, t14107, t14114, t14117)
}
