//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 689/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk689(t3734: f64, t6890: f64, t6889: f64, t22685: f64, t212: f64, t225: f64) -> (f64, f64, f64) {
    let t22686 = t6890 * t3734;
    let t22687 = t6889 * t22686;
    let t22688 = t22685 * t22687;
    let t22690 = t212 * t225;
    (t22686, t22688, t22690)
}
