//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1234/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1234(t6889: f64, t6891: f64, t6888: f64, t117: f64, t534: f64, t67: f64, t6559: f64) -> (f64, f64, f64, f64) {
    let t6892 = t6889 * t6891;
    let t6893 = t6888 * t6892;
    let t6896 = t534 * t67 * t117;
    let t6897 = t6559 * t6896;
    (t6892, t6893, t6896, t6897)
}
