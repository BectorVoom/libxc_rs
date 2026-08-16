//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2871/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2871(t51889: f64, t51919: f64, t51949: f64, t51975: f64, t52009: f64, t52043: f64, t52118: f64, t52134: f64, t964: f64, t973: f64, t981: f64, t11467: f64, t1633: f64, t41235: f64, t41238: f64) -> (f64, f64, f64) {
    let t52137 = t51889 + t51919 + t51949 + t51975 + t52009 + t52043 + t52118 + t52134;
    let t52141 = 0.5848223622634646207e0_f64 * t981 * t964 * t52137 * t973;
    let t52146 = 0.91082604192152556044e5_f64 * t981 * t41235 * t1633 * t41238 * t11467;
    (t52137, t52141, t52146)
}
