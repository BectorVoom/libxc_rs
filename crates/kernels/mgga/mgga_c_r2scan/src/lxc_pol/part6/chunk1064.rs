//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1064/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1064<F: Float>(t357: F, t862: F, t2262: F, t2333: F, t502: F, t57: F, t2206: F, t774: F, t3436: F, t113: F, t1561: F, t978: F, t572: F, t784: F) -> (F, F, F, F, F, F, F, F) {
    let t10645 = t862 * t357;
    let t10687 = t2333 * t2262;
    let t10855 = t502 * t57;
    let t10878 = t2206 * t774;
    let t10979 = t57 * t3436;
    let t11002 = t113 * t1561;
    let t11747 = t2206 * t978;
    let t13866 = t572 * t784;
    (t10645, t10687, t10855, t10878, t10979, t11002, t11747, t13866)
}
