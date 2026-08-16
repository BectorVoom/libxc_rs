//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1729/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1729<F: Float>(t131: F, t557: F, t209: F, t1878: F, t3734: F, t6890: F, t6889: F, t212: F, t225: F) -> (F, F, F, F, F, F, F) {
    let t22683 = t557 * t131;
    let t22684 = t22683 * t209;
    let t22685 = t1878 * t22684;
    let t22686 = t6890 * t3734;
    let t22687 = t6889 * t22686;
    let t22688 = t22685 * t22687;
    let t22690 = t212 * t225;
    (t22683, t22684, t22685, t22686, t22687, t22688, t22690)
}
