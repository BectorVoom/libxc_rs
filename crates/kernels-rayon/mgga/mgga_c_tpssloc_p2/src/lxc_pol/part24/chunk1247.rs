//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1247/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1247(t22662: f64, t22674: f64, t6897: f64, t22684: f64, t6546: f64, t22687: f64, t131: f64, t1365: f64, t1878: f64, t209: f64, t12156: f64, t6889: f64, t6890: f64) -> (f64, f64, f64, f64, f64) {
    let t80725 = t6897 * t22674 * t22662;
    let t80727 = t6546 * t22684;
    let t80728 = t80727 * t22687;
    let t80730 = t1365 * t131;
    let t80732 = t1878 * t80730 * t209;
    let t80735 = t80732 * t6889 * t6890 * t12156;
    (t80725, t80727, t80728, t80732, t80735)
}
