//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 907/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk907<F: Float>(t11059: F, t501: F, t524: F, t305: F, t68: F, t11092: F, t1798: F, t1240: F, t2889: F, t6267: F, t93: F, t1729: F, t1792: F, t2743: F, t11096: F, t1803: F, t1898: F, t2783: F, t455: F, t516: F, t6197: F, t6200: F, t6210: F, t6213: F, t6217: F, t6225: F, t6227: F, t6254: F, t6270: F, t6282: F) -> (F, F, F) {
    let t11122 = t501 * t11059;
    let t11125 = t524 * t11059;
    let t11128 = t305 * t11059;
    let t11129 = t11128 * t68;
    let t11134 = t1798 * t11092;
    let t11140 = t2889 * t1240;
    let t11142 = t6267 * t93 * t11140;
    let t11144 = t2889 * t1729;
    let t11153 = t2743 * t1792;
    let t11154 = t93 * t11153;
    let t11159 = 19.489173774580152 * t11122 * t455 + 18.635258017632964 * t11125 * t455 - 0.04115066352984959 * t11129 * t516 - 2.427516195194328 * t6197 + 2.427516195194328 * t6200 - 2.2140749178833072 * t11134 - 4.4281498357666145 * t1803 * t11096 + 2.2140749178833072 * t1898 * t2783 + 3.5540878740919255 * t11142 - 3.5540878740919255 * t6282 * t93 * t11144 + 4.937333717448355 * t6210 - 4.937333717448355 * t6213 - 0.04115066352984959 * t6217 + 0.04115066352984959 * t6225 + 18.635258017632964 * t6227 - 3.5540878740919255 * t6282 * t11154 + 0.9941357652469939 * t6254 + 3.5540878740919255 * t6270;
    (t11128, t11129, t11159)
}
