//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 389/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk389<F: Float>(t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F, t458: F, t452: F, t1971: F, t462: F, t451: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2075 = 0.7661514025603425 * t1754;
    let t2077 = 0.2553838008534475 * t1765;
    let t2079 = 0.15282509383508946 * t1684;
    let t2081 = 0.05094169794502982 * t1735;
    let t2083 = t2075 - 0.7661514025603425 * t1762 + t2077 + 0.7661514025603425 * t1769 + t2079 - 0.15282509383508946 * t1732 + t2081 + 0.15282509383508946 * t1738;
    let t2084 = t458 * t2083;
    let t2085 = t2084 * t452;
    let t2088 = t462 * t1971;
    let t2091 = t451 * t1971;
    let t2094 = 1.4770435158815312 * t1754;
    let t2096 = 0.49234783862717707 * t1765;
    let t2098 = 0.2946275542389858 * t1684;
    let t2100 = 0.0982091847463286 * t1735;
    let t2102 = t2094 - 1.4770435158815312 * t1762 + t2096 + 1.4770435158815312 * t1769 + t2098 - 0.2946275542389858 * t1732 + t2100 + 0.2946275542389858 * t1738;
    let t2103 = t447 * t2102;
    let t2104 = t2103 * t452;
    (t2075, t2077, t2079, t2081, t2083, t2084, t2085, t2088, t2091, t2094, t2096, t2098, t2100, t2102, t2103, t2104)
}
