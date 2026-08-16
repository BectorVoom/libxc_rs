//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1867/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1867(t1561: f64, t2860: f64, t10760: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t14263: f64, t14266: f64, t14271: f64, t1569: f64, t2863: f64, t2881: f64, t2889: f64, t2907: f64, t4411: f64, t933: f64) -> (f64, f64) {
    let t14276 = t1561 * t2860;
    let t14279 = -0.11696447245269292414e1_f64 * t14263 * t2907 - t13517 - t13519 - t13522 - t13524 - t13526 - t13657 + 2.0_f64 * t14266 * t933 + 1.0_f64 * t4411 * t2881 + 0.32163958997385070134e2_f64 * t14271 * t2889 + 1.0_f64 * t10760 * t1569 - 2.0_f64 * t14276 * t2863;
    (t14276, t14279)
}
