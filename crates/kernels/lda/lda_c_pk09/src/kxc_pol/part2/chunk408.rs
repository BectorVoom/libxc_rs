//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 408/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk408<F: Float>(t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F, t458: F, t452: F, t1971: F, t462: F) -> (F, F, F, F, F, F, F, F) {
    let t2075 = F::cast_from(0.7661514025603425_f64) * t1754;
    let t2077 = F::cast_from(0.2553838008534475_f64) * t1765;
    let t2079 = F::cast_from(0.15282509383508946_f64) * t1684;
    let t2081 = F::cast_from(0.05094169794502982_f64) * t1735;
    let t2083 = t2075 - F::cast_from(0.7661514025603425_f64) * t1762 + t2077 + F::cast_from(0.7661514025603425_f64) * t1769 + t2079 - F::cast_from(0.15282509383508946_f64) * t1732 + t2081 + F::cast_from(0.15282509383508946_f64) * t1738;
    let t2084 = t458 * t2083;
    let t2085 = t2084 * t452;
    let t2088 = t462 * t1971;
    (t2075, t2077, t2079, t2081, t2083, t2084, t2085, t2088)
}
