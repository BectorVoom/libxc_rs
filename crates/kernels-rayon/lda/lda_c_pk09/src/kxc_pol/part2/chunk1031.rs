//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1031/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1031(t1792: f64, t2743: f64, t93: f64, t11096: f64, t11122: f64, t11125: f64, t11129: f64, t11134: f64, t11142: f64, t11144: f64, t1803: f64, t1898: f64, t2783: f64, t455: f64, t516: f64, t6197: f64, t6200: f64, t6210: f64, t6213: f64, t6217: f64, t6225: f64, t6227: f64, t6254: f64, t6270: f64, t6282: f64) -> f64 {
    let t11153 = t2743 * t1792;
    let t11154 = t93 * t11153;
    let t11159 = 19.489173774580152_f64 * t11122 * t455 + 18.635258017632964_f64 * t11125 * t455 - 0.04115066352984959_f64 * t11129 * t516 - 2.427516195194328_f64 * t6197 + 2.427516195194328_f64 * t6200 - 2.2140749178833072_f64 * t11134 - 4.4281498357666145_f64 * t1803 * t11096 + 2.2140749178833072_f64 * t1898 * t2783 + 3.5540878740919255_f64 * t11142 - 3.5540878740919255_f64 * t6282 * t93 * t11144 + 4.937333717448355_f64 * t6210 - 4.937333717448355_f64 * t6213 - 0.04115066352984959_f64 * t6217 + 0.04115066352984959_f64 * t6225 + 18.635258017632964_f64 * t6227 - 3.5540878740919255_f64 * t6282 * t11154 + 0.9941357652469939_f64 * t6254 + 3.5540878740919255_f64 * t6270;
    t11159
}
