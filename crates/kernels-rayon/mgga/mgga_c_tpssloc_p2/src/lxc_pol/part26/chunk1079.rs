//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1079/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1079(t131: f64, t557: f64, t209: f64, t1878: f64, t3734: f64, t6890: f64, t6889: f64, t212: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22683 = t557 * t131;
    let t22684 = t22683 * t209;
    let t22685 = t1878 * t22684;
    let t22686 = t6890 * t3734;
    let t22687 = t6889 * t22686;
    let t22688 = t22685 * t22687;
    let t22690 = t212 * t225;
    (t22683, t22684, t22685, t22686, t22687, t22688, t22690)
}
