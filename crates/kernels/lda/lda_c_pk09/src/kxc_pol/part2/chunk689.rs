//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 689/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk689<F: Float>(t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F, t49: F, t633: F, t2161: F) -> (F, F) {
    let t7848 = -25.0 * t7795 + 25.0 * t7797 + 25.0 * t7799 - 0.8311297508363181 * t7801 - 1.2466946262544771 * t7805 - 1.2466946262544771 * t7809 - 1.2466946262544771 * t7811 - 1.2466946262544771 * t7814 - 1.2466946262544771 * t7817 - 1.2466946262544771 * t7834 - 18.75 * t7838 + 18.75 * t7842 + 18.75 * t7846;
    let t7849 = t49 * t633;
    let t7850 = t7849 * t2161;
    (t7848, t7850)
}
