//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1204/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1204(t1992: f64, t22897: f64, t3792: f64, t74967: f64, t22633: f64, t28116: f64, t90566: f64, t22635: f64, t26331: f64, t26332: f64, t6347: f64, t20356: f64, t6889: f64, t6890: f64, t80732: f64) -> (f64, f64, f64, f64) {
    let t107439 = t1992 * t22897 * t74967 * t3792;
    let t107460 = t22633 * t90566 * t28116;
    let t107464 = t26331 * t22635 * t26332 * t6347;
    let t107484 = t80732 * t6889 * t6890 * t20356;
    (t107439, t107460, t107464, t107484)
}
