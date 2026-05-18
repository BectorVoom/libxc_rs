//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1031/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1031<F: Float>(t1792: F, t2743: F, t93: F, t11096: F, t11122: F, t11125: F, t11129: F, t11134: F, t11142: F, t11144: F, t1803: F, t1898: F, t2783: F, t455: F, t516: F, t6197: F, t6200: F, t6210: F, t6213: F, t6217: F, t6225: F, t6227: F, t6254: F, t6270: F, t6282: F) -> F {
    let t11153 = t2743 * t1792;
    let t11154 = t93 * t11153;
    let t11159 = F::new(19.489173774580152) * t11122 * t455 + F::new(18.635258017632964) * t11125 * t455 - F::new(0.04115066352984959) * t11129 * t516 - F::new(2.427516195194328) * t6197 + F::new(2.427516195194328) * t6200 - F::new(2.2140749178833072) * t11134 - F::new(4.4281498357666145) * t1803 * t11096 + F::new(2.2140749178833072) * t1898 * t2783 + F::new(3.5540878740919255) * t11142 - F::new(3.5540878740919255) * t6282 * t93 * t11144 + F::new(4.937333717448355) * t6210 - F::new(4.937333717448355) * t6213 - F::new(0.04115066352984959) * t6217 + F::new(0.04115066352984959) * t6225 + F::new(18.635258017632964) * t6227 - F::new(3.5540878740919255) * t6282 * t11154 + F::new(0.9941357652469939) * t6254 + F::new(3.5540878740919255) * t6270;
    t11159
}
