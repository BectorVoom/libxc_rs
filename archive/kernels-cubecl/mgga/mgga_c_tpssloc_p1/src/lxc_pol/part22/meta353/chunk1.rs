//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1567/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1567<F: Float>(t16814: F, t17048: F, t858: F, t225: F, t5559: F, t5657: F, t865: F, t2718: F, t17022: F, t218: F, t5636: F, t10110: F) -> (F, F, F, F, F, F, F) {
    let t17049 = t16814 + t17048;
    let t17050 = t858 * t17049;
    let t17052 = t5559 * t225;
    let t17056 = t5657 * t865;
    let t17057 = t2718 * t17056;
    let t17060 = t218 * t17022;
    let t17063 = t5636 * t865;
    let t17064 = t10110 * t17063;
    (t17049, t17050, t17052, t17056, t17057, t17060, t17064)
}
