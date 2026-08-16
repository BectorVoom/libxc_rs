//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1189/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1189(t1799: f64, t22633: f64, t22635: f64, t97608: f64, t1985: f64, t20661: f64, t6889: f64, t6906: f64, t20416: f64, t6888: f64, t6890: f64, t20465: f64, t22833: f64) -> (f64, f64, f64, f64) {
    let t107031 = t22633 * t22635 * t97608 * t1799;
    let t107044 = t1985 * t6889 * t6906 * t20661;
    let t107056 = t6888 * t6889 * t6890 * t20416;
    let t107063 = t22833 * t20465;
    (t107031, t107044, t107056, t107063)
}
