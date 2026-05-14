//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1100/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1100<F: Float>(t2215: F, t3747: F, t836: F, t841: F, t9798: F, t218: F, t3757: F, t675: F, t1167: F, t3026: F, t219: F, t3761: F, t3730: F, t824: F, t334: F, t9795: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9811 = t2215 * t3747;
    let t9812 = t9811 * t836;
    let t9814 = t841 * t9798;
    let t9819 = t218 * t675 * t3757;
    let t9821 = t1167 * t3026;
    let t9823 = t218 * t219 * t9821;
    let t9826 = t218 * t675 * t3761;
    let t9828 = t824 * t3730;
    let t9830 = t218 * t219 * t9828;
    let t9832 = t334 * t9795;
    (t9811, t9812, t9814, t9819, t9821, t9823, t9826, t9828, t9830, t9832)
}
