//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 408/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk408(t1754: f64, t1765: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64, t458: f64, t452: f64, t1971: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2075 = 0.7661514025603425_f64 * t1754;
    let t2077 = 0.2553838008534475_f64 * t1765;
    let t2079 = 0.15282509383508946_f64 * t1684;
    let t2081 = 0.05094169794502982_f64 * t1735;
    let t2083 = t2075 - 0.7661514025603425_f64 * t1762 + t2077 + 0.7661514025603425_f64 * t1769 + t2079 - 0.15282509383508946_f64 * t1732 + t2081 + 0.15282509383508946_f64 * t1738;
    let t2084 = t458 * t2083;
    let t2085 = t2084 * t452;
    let t2088 = t462 * t1971;
    (t2075, t2077, t2079, t2081, t2083, t2084, t2085, t2088)
}
