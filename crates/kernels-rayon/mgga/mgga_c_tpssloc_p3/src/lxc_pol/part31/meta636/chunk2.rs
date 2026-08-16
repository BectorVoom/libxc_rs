//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1903/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1903(t1842: f64, t22633: f64, t22635: f64, t90516: f64, t1992: f64, t26355: f64, t90566: f64, t1307: f64, t26331: f64, t567: f64, t6347: f64, t1985: f64, t20022: f64, t6889: f64, t6906: f64) -> (f64, f64, f64, f64) {
    let t97644 = t22633 * t22635 * t90516 * t1842;
    let t97647 = t1992 * t90566 * t26355;
    let t97652 = t26331 * t22635 * t567 * t6347 * t1307;
    let t97658 = t1985 * t6889 * t6906 * t20022;
    (t97644, t97647, t97652, t97658)
}
