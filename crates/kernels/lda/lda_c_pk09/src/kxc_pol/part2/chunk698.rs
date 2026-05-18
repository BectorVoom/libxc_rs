//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 698/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk698<F: Float>(t1948: F, t6620: F, t1927: F, t6488: F, t1901: F, t6477: F, t490: F, t6601: F, t508: F, t6501: F, t6505: F, t6508: F) -> (F, F, F, F, F, F, F, F) {
    let t6622 = F::new(0.027433775686566395) * t1948 * t6620;
    let t6624 = F::new(12.423505345088643) * t1927 * t6488;
    let t6625 = t1901 * t6477;
    let t6628 = F::new(1.6715885419444727) * t490 * t6601;
    let t6630 = F::new(2.1943705410881575) * t508 * t6601;
    let t6633 = F::new(2.0) * t6501;
    let t6634 = F::new(2.0) * t6505;
    let t6635 = F::new(2.6666666666666665) * t6508;
    (t6622, t6624, t6625, t6628, t6630, t6633, t6634, t6635)
}
