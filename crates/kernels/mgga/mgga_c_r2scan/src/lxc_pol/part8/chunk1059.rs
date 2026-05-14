//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1059/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1059<F: Float>(t10559: F, t10568: F, t10570: F, t10588: F, t354: F, t357: F, t862: F, t502: F, t57: F, t2206: F, t774: F, t3436: F, t113: F, t1561: F, t978: F, t3675: F, t856: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10590 = t10559 + t10568 + t10570 + t10588;
    let t10591 = t354 * t10590;
    let t10645 = t862 * t357;
    let t10855 = t502 * t57;
    let t10878 = t2206 * t774;
    let t10979 = t57 * t3436;
    let t11002 = t113 * t1561;
    let t11747 = t2206 * t978;
    let t11993 = t3675 * t856;
    (t10590, t10591, t10645, t10855, t10878, t10979, t11002, t11747, t11993)
}
