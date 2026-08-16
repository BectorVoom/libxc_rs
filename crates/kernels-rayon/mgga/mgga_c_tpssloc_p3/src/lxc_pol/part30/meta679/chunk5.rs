//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2132/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2132(t22674: f64, t22892: f64, t28209: f64, t22666: f64, t22685: f64, t28191: f64, t6888: f64, t19631: f64, t6889: f64, t6890: f64, t12020: f64, t1385: f64, t1992: f64, t22635: f64, t6439: f64) -> (f64, f64, f64, f64, f64) {
    let t96893 = t22892 * t22674 * t28209;
    let t96896 = t22685 * t22666 * t28191;
    let t96900 = t6888 * t22666 * t28209;
    let t96905 = t6888 * t6889 * t6890 * t19631;
    let t96910 = t1992 * t22635 * t12020 * t6439 * t1385;
    (t96893, t96896, t96900, t96905, t96910)
}
