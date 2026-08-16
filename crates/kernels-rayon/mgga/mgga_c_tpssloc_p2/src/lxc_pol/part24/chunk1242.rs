//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1242/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1242(t12022: f64, t1985: f64, t6889: f64, t80640: f64, t1372: f64, t794: f64, t6897: f64, t6907: f64, t213: f64, t225: f64, t22633: f64, t22637: f64) -> (f64, f64, f64, f64) {
    let t80643 = t1985 * t6889 * t80640 * t12022;
    let t80645 = t794 * t1372;
    let t80647 = t6897 * t80645 * t6907;
    let t80650 = t213 * t1372 * t225;
    let t80652 = t22633 * t80650 * t22637;
    (t80643, t80645, t80647, t80652)
}
