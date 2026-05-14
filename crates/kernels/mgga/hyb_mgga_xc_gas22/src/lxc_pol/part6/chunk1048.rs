//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1048/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1048<F: Float>(t10703: F, t848: F, t4175: F, t839: F, t10743: F, t10746: F, t1359: F, t1371: F, t2246: F, t2285: F, t3366: F, t3386: F, t4154: F, t4167: F, t4170: F, t4194: F, t4197: F, t6636: F, t6678: F, t6722: F, t821: F, t830: F, t840: F, t849: F, t8857: F, t8911: F) -> (F, F, F) {
    let t10759 = t10703 * t848;
    let t10766 = t4175 * t839;
    let t10771 = 1.0 * t821 * t10743 + 1.0 * t10746 * t830 + 2.0 * t8857 * t1359 + 2.0 * t3366 * t3386 - 2.0 * t6722 * t4154 + 1.0 * t2246 * t4167 + 0.5848223622634646207e0 * t2285 * t4194 + 0.5848223622634646207e0 * t840 * t10759 + 0.17315859105681463759e2 * t6636 * t4197 + 0.32163958997385070134e2 * t6678 * t4170 + 0.5848223622634646207e0 * t10766 * t849 + 0.11696447245269292414e1 * t8911 * t1371;
    (t10759, t10766, t10771)
}
