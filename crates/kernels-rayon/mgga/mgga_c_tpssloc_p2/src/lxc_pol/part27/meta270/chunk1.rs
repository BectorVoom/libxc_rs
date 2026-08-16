//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1298/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1298(t6889: f64, t7691: f64, t6888: f64, t1834: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t1842: f64, t6906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7692 = t6889 * t7691;
    let t7693 = t6888 * t7692;
    let t7696 = t1834 * t225 * t567;
    let t7697 = t214 * t7696;
    let t7698 = t1985 * t7697;
    let t7700 = t6906 * t1842;
    (t7692, t7693, t7696, t7697, t7698, t7700)
}
